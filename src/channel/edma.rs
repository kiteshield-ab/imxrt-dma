//! Channel definition, implementation, for eDMA.

use crate::ral::{self, dmamux, tcd::BandwidthControl, Static};
use crate::{Error, SharedWaker};
use cortex_m::interrupt;

use super::{Configuration, DmaChannel};

impl<const CHANNELS: usize> crate::Dma<CHANNELS, ral::dma::edma::RegisterBlock> {
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
    pub(super) index: usize,
    /// Reference to the DMA registers
    registers: Static<crate::ral::dma::edma::RegisterBlock>,
    /// Reference to the DMA multiplexer
    multiplexer: Static<dmamux::RegisterBlock>,
    /// This channel's waker.
    pub(crate) waker: &'static SharedWaker,
}

// It's OK to send a channel across an execution context.
// They can't be cloned or copied, so there's no chance of
// them being (mutably) shared.
unsafe impl Send for Channel {}

impl Channel {
    /// Set the channel's bandwidth control
    ///
    /// - `None` disables bandwidth control (default setting)
    /// - `Some(bwc)` sets the bandwidth control to `bwc`
    ///
    /// Note: This method is not available for eDMA3/eDMA4.
    pub fn set_bandwidth_control(&mut self, bandwidth: Option<BandwidthControl>) {
        let raw = BandwidthControl::raw(bandwidth);
        let tcd = self.tcd();
        crate::ral::modify_reg!(crate::ral::tcd, tcd, CSR, BWC: raw);
    }

    /// Handle a DMA channel interrupt (TODO: Fix this docs)
    ///
    /// Checks the interrupt status for the channel identified by `channel`.
    /// If the channel completed its transfer, `on_interrupt` wakes the channel's
    /// waker.
    ///
    /// Consider calling `on_interrupt` in a DMA channel's interrupt handler:
    ///
    /// ```
    /// use imxrt_dma::DMA;
    /// static DMA: DMA<32> = // Handle to DMA driver.
    /// # unsafe { DMA::new(core::ptr::null(), core::ptr::null()) };
    ///
    /// // #[cortex_m_rt::interrupt]
    /// fn DMA7_DMA23() {
    ///     // Safety: only checking channels 7 and 23, which
    ///     // are both valid on an i.MX RT 1060 chip.
    ///     unsafe {
    ///         DMA.channel(7).on_interrupt();
    ///         DMA.channel(7).on_interrupt();
    ///     }
    /// }
    /// ```
    ///
    /// # Safety
    ///
    /// This should only be used when the associated DMA channel is exclusively referenced
    /// by a DMA transfer future. Caller must ensure that `on_interrupt` is called in
    /// the correct interrupt handler.
    ///
    /// # Panics
    ///
    /// Panics if `channel` is greater than or equal to the maximum number of channels.
    pub unsafe fn on_interrupt(&self) {
        if self.is_interrupt() {
            self.clear_interrupt();
        }

        if self.is_complete() | self.is_error() {
            interrupt::free(|cs| {
                let waker = self.waker.borrow(cs);
                let mut waker = waker.borrow_mut();
                if let Some(waker) = waker.take() {
                    waker.wake();
                }
            });
        }
    }
}

impl DmaChannel for Channel {
    unsafe fn enable(&self) {
        // Immutable write OK. No other methods directly modify ERQ.
        self.registers.SERQ.write(self.index as u8);
    }

    fn tcd(&self) -> &crate::ral::tcd::RegisterBlock {
        &self.registers.TCD[self.index]
    }

    fn set_channel_configuration(&self, configuration: Configuration) {
        // Immutable write OK. 32-bit store on configuration register.
        // eDMA3/4: Haven't found any equivalent to "always on." Doesn't seem
        // that the periodic request via PIT will apply, either.
        //
        // Hardware signals will route to the channel multiplexer configuration
        // register CHn_MUX in the TCD.
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
    }

    fn is_hardware_signaling(&self) -> bool {
        self.registers.HRS.read() & (1 << self.index) != 0
    }

    fn disable(&self) {
        // Immutable write OK. No other methods directly modify ERQ.
        self.registers.CERQ.write(self.index as u8);
    }

    fn is_interrupt(&self) -> bool {
        self.registers.INT.read() & (1 << self.index) != 0
    }

    fn clear_interrupt(&self) {
        // Immutable write OK. No other methods modify INT.
        self.registers.CINT.write(self.index as u8);
    }

    fn is_complete(&self) -> bool {
        let tcd = self.tcd();
        crate::ral::read_reg!(crate::ral::tcd, tcd, CSR, DONE == 1)
    }

    fn clear_complete(&self) {
        // Immutable write OK. CDNE affects a bit in TCD. But, other writes to
        // TCD require &mut reference. Existence of &mut reference blocks
        // clear_complete calls.
        self.registers.CDNE.write(self.index as u8);
    }

    fn is_error(&self) -> bool {
        self.registers.ERR.read() & (1 << self.index) != 0
    }

    fn clear_error(&self) {
        // Immutable write OK. CERR affects a bit in ERR, which is
        // not written to elsewhere.
        self.registers.CERR.write(self.index as u8);
    }

    fn is_active(&self) -> bool {
        let tcd = self.tcd();
        ral::read_reg!(crate::ral::tcd, tcd, CSR, ACTIVE == 1)
    }

    fn is_enabled(&self) -> bool {
        self.registers.ERQ.read() & (1 << self.index) != 0
    }

    fn error_status(&self) -> Error {
        Error::new(self.registers.ES.read())
    }

    fn channel(&self) -> usize {
        self.index
    }

    fn waker(&self) -> &SharedWaker {
        self.waker
    }
}
