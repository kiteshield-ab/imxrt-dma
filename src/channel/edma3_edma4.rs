//! Channel definitions, implementation, for eDMA3 / eDMA4.

use cortex_m::interrupt;

use crate::ral::{self, dma, Static};
use crate::{Error, SharedWaker};

use super::{Configuration, DmaChannel};

impl<const CHANNELS: usize> crate::Dma<CHANNELS, dma::edma3::RegisterBlock> {
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
    pub unsafe fn channel(&'static self, index: usize) -> Channel<dma::edma3::RegisterBlock> {
        assert!(index < CHANNELS);
        Channel {
            index,
            registers: self.controller,
            waker: &self.wakers[index],
        }
    }
}

impl<const CHANNELS: usize> crate::Dma<CHANNELS, dma::edma4::RegisterBlock> {
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
    pub unsafe fn channel(&'static self, index: usize) -> Channel<dma::edma4::RegisterBlock> {
        assert!(index < CHANNELS);
        Channel {
            index,
            registers: self.controller,
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
pub struct Channel<DmaPeripheral> {
    /// Our channel number, expected to be between [0, 32)
    pub(super) index: usize,
    /// Reference to the DMA registers
    registers: Static<DmaPeripheral>,
    /// This channel's waker.
    pub(crate) waker: &'static SharedWaker,
}

// It's OK to send a channel across an execution context.
// They can't be cloned or copied, so there's no chance of
// them being (mutably) shared.
unsafe impl<T> Send for Channel<T> {}

/// Specifics needed for eDMA3/4.
pub trait Dma3Dma4Specifics {
    /// Get the registers of the specific channel.
    fn channel_registers(&self, index: usize) -> &ral::tcd::edma34::RegisterBlock;

    /// DMA dependent hardware signaling check.
    fn is_hardware_signaling(&self, index: usize) -> bool;
}

impl<DmaPeripheral> DmaChannel for Channel<DmaPeripheral>
where
    DmaPeripheral: Dma3Dma4Specifics,
{
    /// Returns a handle to this channel's transfer control descriptor.
    fn tcd(&self) -> &crate::ral::tcd::RegisterBlock {
        &self.registers.channel_registers(self.index).TCD
    }

    fn is_hardware_signaling(&self) -> bool {
        self.registers.is_hardware_signaling(self.index)
    }

    unsafe fn enable(&self) {
        // eDMA3/4: dispatch to the TCD CHn_CSR. RMW on bit
        // 0 to enable. Immutable write still OK: channel
        // deemed unique, and it should be !Sync.
        let chan = self.registers.channel_registers(self.index);
        ral::modify_reg!(crate::ral::tcd::edma34, chan, CSR, ERQ: 1);
    }

    fn set_channel_configuration(&self, configuration: Configuration) {
        let source = match configuration {
            Configuration::Off => 0,
            Configuration::Enable { source } => source,
        };
        let chan = self.registers.channel_registers(self.index);
        ral::write_reg!(crate::ral::tcd::edma34, chan, MUX, source);
    }

    fn disable(&self) {
        // eDMA3/4: see notes in enable. RMW to set bit 0 low.
        let chan = self.registers.channel_registers(self.index);
        ral::modify_reg!(crate::ral::tcd::edma34, chan, CSR, ERQ: 0);
    }

    fn is_interrupt(&self) -> bool {
        // eDMA3/4: Each channel has a W1C interrupt bit.
        // Prefer that instead of the aggregate register(s)
        // in the MP space.
        self.registers.channel_registers(self.index).INT.read() != 0
    }

    fn clear_interrupt(&self) {
        // eDMA3/4: See note in is_interrupt.
        self.registers.channel_registers(self.index).INT.write(1);
    }

    fn is_complete(&self) -> bool {
        // eDMA3/4: Need to check CHn_CSR in the TCD space.
        let chan = self.registers.channel_registers(self.index);
        ral::read_reg!(crate::ral::tcd::edma34, chan, CSR, DONE == 1)
    }

    fn clear_complete(&self) {
        // eDMA3/4: Need to change a CHn_CSR bit in the TCD space.
        let chan = self.registers.channel_registers(self.index);
        ral::modify_reg!(crate::ral::tcd::edma34, chan, CSR, DONE: 1);
    }

    fn is_error(&self) -> bool {
        // eDMA3/4: Check CHn_ES, highest bit.
        self.registers.channel_registers(self.index).ES.read() != 0
    }

    fn clear_error(&self) {
        // eDMA3/4: W1C CHn_ES, highest bit.
        self.registers
            .channel_registers(self.index)
            .ES
            .write(1 << 31);
    }

    fn is_active(&self) -> bool {
        // eDMA3/4: Check CHn_CSR, highest bit.
        let chan = self.registers.channel_registers(self.index);
        ral::read_reg!(crate::ral::tcd::edma34, chan, CSR, ACTIVE == 1)
    }

    fn is_enabled(&self) -> bool {
        // eDMA3/4: Check CHn_CSR, lowest bit.
        let chan = self.registers.channel_registers(self.index);
        ral::read_reg!(crate::ral::tcd::edma34, chan, CSR, ERQ == 1)
    }

    fn error_status(&self) -> Error {
        Error::new(self.registers.channel_registers(self.index).ES.read())
    }

    fn channel(&self) -> usize {
        self.index
    }

    fn waker(&self) -> &SharedWaker {
        self.waker
    }
}

impl<DmaPeripheral> Channel<DmaPeripheral>
where
    DmaPeripheral: Dma3Dma4Specifics,
{
    /// Handle a DMA channel interrupt (TODO: Fix this docs)
    ///
    /// Checks the interrupt status for the channel identified by `channel`.
    /// If the channel completed its transfer, `on_interrupt` wakes the channel's
    /// waker.
    ///
    /// Consider calling `on_interrupt` in a DMA channel's interrupt handler:
    ///
    /// ```
    /// use imxrt_dma::DMA3;
    /// static DMA: DMA3 = // Handle to DMA driver.
    /// # unsafe { DMA3::new_edma3(core::ptr::null()) };
    ///
    /// // #[cortex_m_rt::interrupt]
    /// fn DMA7_DMA23() {
    ///     // Safety: only checking channels 7 and 23, which
    ///     // are both valid on an i.MX RT 1060 chip.
    ///     unsafe {
    ///         DMA.channel(7).on_interrupt();
    ///         DMA.channel(23).on_interrupt();
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

    /// Adopt and use the domain ID of the bus controller to perform access.
    ///
    /// Only matters if the global ID replication is enabled in the DMA controller.
    pub fn set_id_replication(&mut self, enable: bool) {
        let chan = self.registers.channel_registers(self.index);
        ral::modify_reg!(crate::ral::tcd::edma34, chan, SBR, EMI: enable as u32);
    }

    /// Promote the channel's protection level to privileged.
    ///
    /// If not enabled, the privilege level is nonprivilaged, or
    /// "user protection."
    pub fn set_privilege_protection(&mut self, enable: bool) {
        let chan = self.registers.channel_registers(self.index);
        ral::modify_reg!(crate::ral::tcd::edma34, chan, SBR, PAL: enable as u32);
    }

    /// Promote the channel's protection level to sercure.
    ///
    /// If not enabled, accesses adopt nonsecure protection.
    pub fn set_secure_protection(&mut self, enable: bool) {
        let chan = self.registers.channel_registers(self.index);
        ral::modify_reg!(crate::ral::tcd::edma34, chan, SBR, SEC: enable as u32);
    }
}

//
// Specifics for DMA3 and DMA4.
//

impl Dma3Dma4Specifics for dma::edma3::RegisterBlock {
    fn channel_registers(&self, index: usize) -> &ral::tcd::edma34::RegisterBlock {
        &self.TCD[index]
    }

    fn is_hardware_signaling(&self, index: usize) -> bool {
        self.HRS.read() & 1 << index != 0
    }
}

impl Dma3Dma4Specifics for dma::edma4::RegisterBlock {
    fn channel_registers(&self, index: usize) -> &ral::tcd::edma34::RegisterBlock {
        &self.TCD[index]
    }

    fn is_hardware_signaling(&self, index: usize) -> bool {
        if index < 32 {
            self.HRS_LOW.read() & 1 << index != 0
        } else {
            self.HRS_HIGH.read() & 1 << (index - 32) != 0
        }
    }
}
