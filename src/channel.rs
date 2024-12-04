//! DMA channels
//!
//! `channel` contains the DMA [`Channel`] type, along with helper functions for
//! defining transfers.
//!
//! `Channel` methods that specify memory involved in transfers are marked `unsafe`. You must
//! be very careful when calling these methods, particuarly when a channel is already
//! enabled.

use crate::{element::Element, ral, Error};

#[cfg(not(feature = "edma34"))]
use crate::ral::{dmamux, tcd::BandwidthControl, Static};

use cfg_if::cfg_if;

impl<const CHANNELS: usize> super::Dma<CHANNELS> {
    /// Creates the DMA channel described by `index`.
    ///
    /// # Safety
    ///
    /// This will create a handle that may alias global, mutable state. You should only create
    /// one channel per index. If there are multiple channels for the same index, you're
    /// responsible for ensuring synchronized access.
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than or equal to the maximum number of channels.
    pub unsafe fn channel(&'static self, index: usize) -> Channel {
        assert!(index < CHANNELS);
        Channel {
            index,
            registers: self.controller,
            #[cfg(not(feature = "edma34"))]
            multiplexer: self.multiplexer,
            waker: &self.wakers[index],
        }
    }
}

/// A DMA channel
///
/// You should rely on your HAL to allocate `Channel`s. If your HAL does not allocate channels,
/// or if you're desigining the HAL, use [`Dma`](crate::Dma) to create channels.
///
/// The `Channel` stores memory addresses independent of the memory lifetime. You must make
/// sure that the channel's state is valid before enabling a transfer!
pub struct Channel {
    /// Our channel number, expected to be between [0, 32)
    index: usize,
    /// Reference to the DMA registers
    registers: ral::Kind,
    /// Reference to the DMA multiplexer
    #[cfg(not(feature = "edma34"))]
    multiplexer: Static<dmamux::RegisterBlock>,
    /// This channel's waker.
    pub(crate) waker: &'static super::SharedWaker,
}

impl Channel {
    /// Enable the DMA channel for transfers
    ///
    /// # Safety
    ///
    /// `enable()` allows the DMA controller to read and write from the memory
    /// addresses stored in the channel. This is very unsafe:
    ///
    /// - you must ensure that the lifetime of all memory is greater than the
    ///   lifetime of the transfer
    /// - you must ensure that no one else is using this channel for anything
    ///   else
    /// - if the transfer uses a circular buffer, you must ensure that the circular
    ///   buffer is correctly sized and aligned.
    pub unsafe fn enable(&self) {
        // Immutable write OK. No other methods directly modify ERQ.
        cfg_if! {
            if #[cfg(not(feature = "edma34"))] {
                self.registers.SERQ.write(self.index as u8);
            } else {
                // eDMA3/4: dispatch to the TCD CHn_CSR. RMW on bit
                // 0 to enable. Immutable write still OK: channel
                // deemed unique, and it should be !Sync.
                let chan = self.registers.channel(self.index);
                ral::modify_reg!(crate::ral::tcd::edma34, chan, CSR, ERQ: 1);
            }
        }
    }

    /// Returns the DMA channel number
    ///
    /// Channels are unique and numbered within the half-open range `[0, 32)`.
    pub fn channel(&self) -> usize {
        self.index
    }

    /// Set the channel's bandwidth control
    ///
    /// - `None` disables bandwidth control (default setting)
    /// - `Some(bwc)` sets the bandwidth control to `bwc`
    #[cfg(not(feature = "edma34"))] // TODO: Could be supported for eDMA3.
    pub fn set_bandwidth_control(&mut self, bandwidth: Option<BandwidthControl>) {
        let raw = BandwidthControl::raw(bandwidth);
        let tcd = self.tcd();
        ral::modify_reg!(crate::ral::tcd, tcd, CSR, BWC: raw);
        // eDMA4: Not valid, and not sure where anything similar
        // is. Could just remove, or return a "not supported" error.
    }

    /// Reset the transfer control descriptor owned by the DMA channel
    ///
    /// `reset` should be called during channel initialization to put the
    /// channel into a known, good state.
    pub fn reset(&mut self) {
        self.tcd().reset();
    }

    /// Returns a handle to this channel's transfer control descriptor
    fn tcd(&self) -> &crate::ral::tcd::RegisterBlock {
        self.registers.tcd(self.index)
    }

    /// Set the source address for a DMA transfer
    ///
    /// `saddr` should be a memory location that can provide the DMA controller
    /// with data.
    ///
    /// # Safety
    ///
    /// If the DMA channel is already enabled, the DMA engine may start reading this
    /// memory location. You must ensure that reads to `saddr` do not perform
    /// inappropriate side effects. You must ensure `saddr` is valid for the
    /// lifetime of the transfer.
    pub unsafe fn set_source_address<E: Element>(&self, saddr: *const E) {
        // Immutable write OK. 32-bit aligned store on SADDR.
        let tcd = self.tcd();
        ral::write_reg!(crate::ral::tcd, tcd, SADDR, saddr as u32);
    }

    /// Set the source offset *in bytes*
    ///
    /// `offset` could be negative, which would decrement the address.
    ///
    /// # Safety
    ///
    /// This method could allow a DMA engine to read beyond a buffer or
    /// address. You must ensure that the source is valid for these offsets.
    pub unsafe fn set_source_offset(&self, offset: i16) {
        // Immutable write OK. 16-bit aligned store on SOFF.
        let tcd = self.tcd();
        ral::write_reg!(crate::ral::tcd, tcd, SOFF, offset);
    }

    /// Set the destination address for a DMA transfer
    ///
    /// `daddr` should be a memory location that can store data from the
    /// DMA controller.
    ///
    /// # Safety
    ///
    /// If the DMA channel is already enabled, the DMA engine may start
    /// writing to this address. You must ensure that writes to `daddr`
    /// are safe, and that the memory is valid for the lifetime of the
    /// transfer.
    pub unsafe fn set_destination_address<E: Element>(&self, daddr: *const E) {
        // Immutable write OK. 32-bit aligned store on DADDR.
        let tcd = self.tcd();
        ral::write_reg!(crate::ral::tcd, tcd, DADDR, daddr as u32);
    }

    /// Set the destination offset *in bytes*
    ///
    /// `offset` could be negative, which would decrement the address.
    ///
    /// # Safety
    ///
    /// This method could allow a DMA engine to write beyond the range of
    /// a buffer. You must ensure that the destination is valid for these
    /// offsets.
    pub unsafe fn set_destination_offset(&self, offset: i16) {
        // Immutable write OK. 16-bit aligned store on DOFF.
        let tcd = self.tcd();
        ral::write_reg!(crate::ral::tcd, tcd, DOFF, offset);
    }

    /// Set the transfer attributes for the source
    ///
    /// # Safety
    ///
    /// An incorrect `modulo` value may allow the DMA engine to loop back
    /// to an incorrect address. You must ensure that `modulo` is valid
    /// for your source.
    pub unsafe fn set_source_attributes<E: Element>(&self, modulo: u8) {
        let tcd = self.tcd();
        ral::write_reg!(
            crate::ral::tcd,
            tcd,
            SATTR,
            MOD: modulo,
            SIZE: E::DATA_TRANSFER_ID
        );
    }

    /// Set the source last address adjustment *in bytes*
    ///
    /// # Safety
    ///
    /// This could allow the DMA engine to reference an invalid source buffer.
    /// You must ensure that the adjustment performed by the DMA engine is
    /// valid, assuming that another DMA transfer immediately runs after the
    /// current transfer completes.
    pub unsafe fn set_source_last_address_adjustment(&self, adjustment: i32) {
        let tcd = self.tcd();
        ral::write_reg!(crate::ral::tcd, tcd, SLAST, adjustment);
    }

    /// Set the destination last addrss adjustment *in bytes*
    ///
    /// # Safety
    ///
    /// This could allow the DMA engine to reference an invalid destination address.
    /// You must ensure that the adjustment performed by the DMA engine is
    /// valid, assuming that another DMA transfer immediately runs after the
    /// current transfer completes.
    pub unsafe fn set_destination_last_address_adjustment(&self, adjustment: i32) {
        let tcd = self.tcd();
        ral::write_reg!(crate::ral::tcd, tcd, DLAST_SGA, adjustment);
    }

    /// Set the transfer attributes for the destination
    ///
    /// # Safety
    ///
    /// An incorrect `modulo` value may allow the DMA engine to loop back
    /// to an incorrect address. You must ensure that `modulo` is valid
    /// for your destination.
    pub unsafe fn set_destination_attributes<E: Element>(&self, modulo: u8) {
        let tcd = self.tcd();
        ral::write_reg!(
            crate::ral::tcd,
            tcd,
            DATTR,
            MOD: modulo,
            SIZE: E::DATA_TRANSFER_ID
        );
    }

    /// Set the number of *bytes* to transfer per minor loop
    ///
    /// Describes how many bytes we should transfer for each DMA service request.
    /// Note that `nbytes` of `0` is interpreted as a 4GB transfer.
    ///
    /// # Safety
    ///
    /// This might allow the DMA engine to read beyond the source, or write beyond
    /// the destination. Caller must ensure that the number of bytes per minor loop
    /// is valid for the given transfer.
    pub unsafe fn set_minor_loop_bytes(&self, nbytes: u32) {
        // Immutable write OK. 32-bit store on NBYTES.
        let tcd = self.tcd();
        ral::write_reg!(crate::ral::tcd, tcd, NBYTES, nbytes);
    }

    /// Tells the DMA channel how many transfer iterations to perform
    ///
    /// A 'transfer iteration' is a read from a source, and a write to a destination, with
    /// read and write sizes described by a minor loop. Each iteration requires a DMA
    /// service request, either from hardware or from software. The maximum number of iterations
    /// is 2^15.
    ///
    /// # Safety
    ///
    /// This may allow the DMA engine to read beyond the source, or write beyond
    /// the destination. Caller must ensure that the number of iterations is valid
    /// for the transfer.
    pub unsafe fn set_transfer_iterations(&mut self, iterations: u16) {
        let tcd = self.tcd();
        // Note that this is clearing the ELINK bit. We don't have support
        // for channel-to-channel linking right now. Clearing ELINK is intentional
        // to use the whole 15 bits for iterations.
        ral::modify_reg!(crate::ral::tcd, tcd, CITER, CITER: iterations);
        ral::modify_reg!(crate::ral::tcd, tcd, BITER, BITER: iterations);
    }

    /// Returns the beginning transfer iterations setting for the channel.
    ///
    /// This reflects the last call to `set_transfer_iterations`.
    pub fn beginning_transfer_iterations(&self) -> u16 {
        let tcd = self.tcd();
        ral::read_reg!(crate::ral::tcd, tcd, BITER, BITER)
    }

    /// Set the DMAMUX channel configuration
    ///
    /// See the [`Configuration`](crate::channel::Configuration) documentation
    /// for more information.
    ///
    /// # Panics
    ///
    /// Only the first four DMA channels support periodic triggering from PIT timers. This method
    /// panics if `triggering` is set for the [`Enable`](crate::channel::Configuration)
    /// variant, but the channel does not support triggering.
    pub fn set_channel_configuration(&mut self, configuration: Configuration) {
        // Immutable write OK. 32-bit store on configuration register.
        // eDMA3/4: Haven't found any equivalent to "always on." Doesn't seem
        // that the periodic request via PIT will apply, either.
        //
        // Hardware signals will route to the channel multiplexer configuration
        // register CHn_MUX in the TCD.
        cfg_if! {
            if #[cfg(not(feature = "edma34"))] {
                let chcfg = &self.multiplexer.chcfg[self.index];
                match configuration {
                    Configuration::Off => chcfg.write(0),
                    Configuration::Enable { source, periodic } => {
                        let mut v = source | dmamux::RegisterBlock::ENBL;
                        if periodic {
                            assert!(
                                self.channel() < 4,
                                "Requested DMA periodic triggering on an unsupported channel."
                            );
                            v |= dmamux::RegisterBlock::TRIG;
                        }
                        chcfg.write(v);
                    }
                    Configuration::AlwaysOn => {
                        // See note in reference manual: when A_ON is high, SOURCE is ignored.
                        chcfg.write(dmamux::RegisterBlock::ENBL | dmamux::RegisterBlock::A_ON)
                    }
                }
            } else {
                let source = match configuration {
                    Configuration::Off => 0,
                    Configuration::Enable { source } => source,
                };
                let chan = self.registers.channel(self.index);
                ral::write_reg!(crate::ral::tcd::edma34, chan, MUX, source);
            }
        }
    }

    /// Returns `true` if the DMA channel is receiving a service signal from hardware
    pub fn is_hardware_signaling(&self) -> bool {
        cfg_if! {
            if #[cfg(not(feature = "edma34"))] {
                self.registers.HRS.read() & (1 << self.index) != 0
            } else {
                // eDMA4: Since there's so many channels, there's an extra
                // register we need to be aware of.
                self.registers.is_hardware_signaling(self.index)
            }
        }
    }

    /// Disable the DMA channel, preventing any DMA transfers
    pub fn disable(&self) {
        // Immutable write OK. No other methods directly modify ERQ.
        cfg_if! {
            if #[cfg(not(feature = "edma34"))] {
                self.registers.CERQ.write(self.index as u8);
            } else {
                // eDMA3/4: see notes in enable. RMW to set bit 0 low.
                let chan = self.registers.channel(self.index);
                ral::modify_reg!(crate::ral::tcd::edma34, chan, CSR, ERQ: 0);
            }
        }
    }

    /// Returns `true` if this DMA channel generated an interrupt
    pub fn is_interrupt(&self) -> bool {
        cfg_if! {
            if #[cfg(not(feature = "edma34"))] {
                self.registers.INT.read() & (1 << self.index) != 0
            } else {
                // eDMA3/4: Each channel has a W1C interrupt bit.
                // Prefer that instead of the aggregate register(s)
                // in the MP space.
                self.registers.channel(self.index).INT.read() != 0
            }
        }
    }

    /// Clear the interrupt flag from this DMA channel
    pub fn clear_interrupt(&self) {
        cfg_if! {
            if #[cfg(not(feature = "edma34"))] {
                // Immutable write OK. No other methods modify INT.
                self.registers.CINT.write(self.index as u8);
            } else {
                // eDMA3/4: See note in is_interrupt.
                self.registers.channel(self.index).INT.write(1);
            }
        }
    }

    /// Enable or disable 'disable on completion'
    ///
    /// 'Disable on completion' lets the DMA channel automatically clear the request signal
    /// when it completes a transfer.
    pub fn set_disable_on_completion(&mut self, dreq: bool) {
        let tcd = self.tcd();
        ral::modify_reg!(crate::ral::tcd, tcd, CSR, DREQ: dreq as u16);
    }

    /// Enable or disable interrupt generation when the transfer completes
    ///
    /// You're responsible for registering your interrupt handler.
    pub fn set_interrupt_on_completion(&mut self, intr: bool) {
        let tcd = self.tcd();
        ral::modify_reg!(crate::ral::tcd, tcd, CSR, INTMAJOR: intr as u16);
    }

    /// Indicates if the DMA transfer has completed
    pub fn is_complete(&self) -> bool {
        cfg_if! {
            if #[cfg(not(feature = "edma34"))] {
                let tcd = self.tcd();
                ral::read_reg!(crate::ral::tcd, tcd, CSR, DONE == 1)
            } else {
                // eDMA3/4: Need to check CHn_CSR in the TCD space.
                let chan = self.registers.channel(self.index);
                ral::read_reg!(crate::ral::tcd::edma34, chan, CSR, DONE == 1)
            }
        }
    }

    /// Clears completion indication
    pub fn clear_complete(&self) {
        cfg_if! {
            if #[cfg(not(feature = "edma34"))] {
                // Immutable write OK. CDNE affects a bit in TCD. But, other writes to
                // TCD require &mut reference. Existence of &mut reference blocks
                // clear_complete calls.
                self.registers.CDNE.write(self.index as u8);
            } else {
                // eDMA3/4: Need to change a CHn_CSR bit in the TCD space.
                let chan = self.registers.channel(self.index);
                ral::modify_reg!(crate::ral::tcd::edma34, chan, CSR, DONE: 1);
            }
        }
    }

    /// Indicates if the DMA channel is in an error state
    pub fn is_error(&self) -> bool {
        cfg_if! {
            if #[cfg(not(feature = "edma34"))] {
                self.registers.ERR.read() & (1 << self.index) != 0
            } else {
                // eDMA3/4: Check CHn_ES, highest bit.
                self.registers.channel(self.index).ES.read() != 0
            }
        }
    }

    /// Clears the error flag
    pub fn clear_error(&self) {
        cfg_if! {
            if #[cfg(not(feature = "edma34"))] {
                // Immutable write OK. CERR affects a bit in ERR, which is
                // not written to elsewhere.
                self.registers.CERR.write(self.index as u8);
            } else {
                // eDMA3/4: W1C CHn_ES, highest bit.
                self.registers.channel(self.index).ES.write(1 << 31);
            }
        }
    }

    /// Indicates if this DMA channel is actively transferring data
    pub fn is_active(&self) -> bool {
        cfg_if! {
            if #[cfg(not(feature = "edma34"))] {
                let tcd = self.tcd();
                ral::read_reg!(crate::ral::tcd, tcd, CSR, ACTIVE == 1)
            } else {
                // eDMA3/4: Check CHn_CSR, highest bit.
                let chan = self.registers.channel(self.index);
                ral::read_reg!(crate::ral::tcd::edma34, chan, CSR, ACTIVE == 1)
            }
        }
    }

    /// Indicates if this DMA channel is enabled
    pub fn is_enabled(&self) -> bool {
        cfg_if! {
            if #[cfg(not(feature = "edma34"))] {
                self.registers.ERQ.read() & (1 << self.index) != 0
            } else {
                // eDMA3/4: Check CHn_CSR, lowest bit.
                let chan = self.registers.channel(self.index);
                ral::read_reg!(crate::ral::tcd::edma34, chan, CSR, ERQ == 1)
            }
        }
    }

    /// Returns the value from the **global** error status register
    ///
    /// It may reflect the last channel that produced an error, and that
    /// may not be related to this channel.
    pub fn error_status(&self) -> Error {
        cfg_if! {
            if #[cfg(not(feature = "edma34"))] {
                Error::new(self.registers.ES.read())
            } else {
                // eDMA3/4: Read CHn_ES.
                Error::new(self.registers.channel(self.index).ES.read())
            }
        }
    }

    /// Start a DMA transfer
    ///
    /// `start()` should be used to request service from the DMA controller. It's
    /// necessary for in-memory DMA transfers. Do not use it for hardware-initiated
    /// DMA transfers. DMA transfers that involve hardware will rely on the hardware
    /// to request DMA service.
    ///
    /// Flag is automatically cleared by hardware after it's asserted.
    pub fn start(&self) {
        let tcd = self.tcd();
        ral::modify_reg!(crate::ral::tcd, tcd, CSR, START: 1);
    }
}

// It's OK to send a channel across an execution context.
// They can't be cloned or copied, so there's no chance of
// them being (mutably) shared.
unsafe impl Send for Channel {}

/// DMAMUX channel configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Configuration {
    /// The DMAMUX channel is disabled
    Off,
    /// The DMAMUX is enabled, permitting hardware triggering.
    /// See [`enable`](Configuration::enable) to enable
    /// the channel without periodic triggering.
    Enable {
        /// The DMA channel source (slot number)
        ///
        /// Specifies which DMA source is routed to the DMA channel.
        source: u32,
        /// Set the periodic triggering flag to schedule DMA transfers on PIT
        /// timer scheduling.
        ///
        /// `periodic` only works for the first four DMA channels, since
        /// it corresponds to the PIT timers.
        #[cfg(not(feature = "edma34"))]
        periodic: bool,
    },
    /// The DMAMUX is always on, and there's no need for software
    /// or hardware activation
    ///
    /// Use `AlwaysOn` for
    /// - memory-to-memory transfers
    /// - memory to external bus transfers
    #[cfg(not(feature = "edma34"))]
    AlwaysOn,
}

impl Configuration {
    /// Enable the channel without triggering
    ///
    /// Shorthand for `ChannelConfiguration::Enable { source, periodic: false }`.
    /// Use `enable()` to avoid possible panics in
    /// [`set_channel_configuration`](crate::channel::Channel::set_channel_configuration).
    pub const fn enable(source: u32) -> Self {
        Configuration::Enable {
            source,
            #[cfg(not(feature = "edma34"))]
            periodic: false,
        }
    }
}

/// Set a hardware peripheral as the source for a DMA transfer
///
/// `hardware_source` is expected to be a pointer to a peripheral register that
/// can provide DMA data. This function configures the DMA channel always read from
/// this register.
///
/// # Safety
///
/// Caller must ensure that `hardware_source` is valid for the lifetime of the transfer,
/// and valid for all subsequent transfers performed by this DMA channel with this address.
pub unsafe fn set_source_hardware<E: Element>(chan: &mut Channel, hardware_source: *const E) {
    chan.set_source_address(hardware_source);
    chan.set_source_offset(0);
    chan.set_source_attributes::<E>(0);
    chan.set_source_last_address_adjustment(0);
}

/// Set a hardware peripheral as the destination for a DMA transfer
///
/// `hardware_destination` is expected to point at a peripheral register that can
/// receive DMA data. This function configures the DMA channel to always write to
/// this register.
///
/// # Safety
///
/// Caller must ensure that `hardware_destination` is valid for the lifetime of the transfer,
/// and valid for all subsequent transfers performed by this DMA channel with this address.
pub unsafe fn set_destination_hardware<E: Element>(
    chan: &mut Channel,
    hardware_destination: *const E,
) {
    chan.set_destination_address(hardware_destination);
    chan.set_destination_offset(0);
    chan.set_destination_attributes::<E>(0);
    chan.set_destination_last_address_adjustment(0);
}

/// Set a linear buffer as the source for a DMA transfer
///
/// When the transfer completes, the DMA channel will point at the
/// start of the buffer.
///
/// # Safety
///
/// Caller must ensure that the source is valid for the lifetime of the transfer,
/// and valid for all subsequent transfers performed by this DMA channel with this buffer.
pub unsafe fn set_source_linear_buffer<E: Element>(chan: &mut Channel, source: &[E]) {
    chan.set_source_address(source.as_ptr());
    chan.set_source_offset(core::mem::size_of::<E>() as i16);
    chan.set_source_attributes::<E>(0);
    chan.set_source_last_address_adjustment((core::mem::size_of_val(source) as i32).wrapping_neg());
}

/// Set a linear buffer as the destination for a DMA transfer
///
/// When the transfer completes, the DMA channel will point at the
/// start of the buffer.
///
/// # Safety
///
/// Caller must ensure that the destination is valid for the lifetime of the transfer,
/// and valid for all subsequent transfers performed by this DMA channel with this buffer.
pub unsafe fn set_destination_linear_buffer<E: Element>(chan: &mut Channel, destination: &mut [E]) {
    chan.set_destination_address(destination.as_ptr());
    chan.set_destination_offset(core::mem::size_of::<E>() as i16);
    chan.set_destination_attributes::<E>(0);
    chan.set_destination_last_address_adjustment(
        (core::mem::size_of_val(destination) as i32).wrapping_neg(),
    );
}

/// Assert properties about the circular buffer
fn circular_buffer_asserts<E>(buffer: &[E]) {
    let len = buffer.len();
    assert!(
        len.is_power_of_two(),
        "DMA circular buffer size is not power of two"
    );
    let start = buffer.as_ptr();
    let size = core::mem::size_of_val(buffer);
    assert!(
        (start as usize) % size == 0,
        "DMA circular buffer is not properly aligned"
    );
}

/// Compute the circular buffer modulo value
fn circular_buffer_modulo<E>(buffer: &[E]) -> u32 {
    31 - core::mem::size_of_val(buffer).leading_zeros()
}

/// Set a circular buffer as the source for a DMA transfer
///
/// When the transfer completes, the DMA channel remain at the
/// next element in the circular buffer.
///
/// # Safety
///
/// Caller must ensure that the source is valid for the lifetime of the transfer,
/// and for all subsequent transfers performed by this DMA channel with this buffer.
///
/// # Panics
///
/// Panics if
///
/// - the capacity is not a power of two
/// - the alignment is not a multiple of the buffer's size in bytes
pub unsafe fn set_source_circular_buffer<E: Element>(chan: &mut Channel, source: &[E]) {
    circular_buffer_asserts(source);
    let modulo = circular_buffer_modulo(source);

    chan.set_source_address(source.as_ptr());
    chan.set_source_offset(core::mem::size_of::<E>() as i16);
    chan.set_source_attributes::<E>(modulo as u8);
    chan.set_source_last_address_adjustment(0);
}

/// Set a circular buffer as the destination for a DMA transfer
///
/// When the transfer completes, the DMA channel remain at the
/// next element in the circular buffer.
///
/// # Safety
///
/// Caller must ensure that the destination is valid for the lifetime of the transfer,
/// and for all subsequent transfers performed by this DMA channel with this buffer.
///
/// # Panics
///
/// Panics if
///
/// - the capacity is not a power of two
/// - the alignment is not a multiple of the buffer's size in bytes
pub unsafe fn set_destination_circular_buffer<E: Element>(
    chan: &mut Channel,
    destination: &mut [E],
) {
    circular_buffer_asserts(destination);
    let modulo = circular_buffer_modulo(destination);

    chan.set_destination_address(destination.as_ptr());
    chan.set_destination_offset(core::mem::size_of::<E>() as i16);
    chan.set_destination_attributes::<E>(modulo as u8);
    chan.set_destination_last_address_adjustment(0);
}
