use ral_registers::{RORegister, RWRegister};

/// DMA MP
#[repr(C)]
pub struct RegisterBlock {
    /// Management Page Control
    pub MP_CSR: RWRegister<u32>,
    /// Management Page Error Status
    pub MP_ES: RORegister<u32>,
    /// Management Page Interrupt Request Status - Low
    pub MP_INT_LOW: RORegister<u32>,
    /// Management Page Interrupt Request Status - High
    pub MP_INT_HIGH: RORegister<u32>,
    /// Management Page Hardware Request Status - Low
    pub MP_HRS_LOW: RORegister<u32>,
    /// Management Page Hardware Request Status - High
    pub MP_HRS_HIGH: RORegister<u32>,
    _reserved0: [u8; 0xe8],
    /// Channel Arbitration Group
    pub CH_GRPRI: [RWRegister<u32>; 64usize],
}

/// Management Page Control
pub mod MP_CSR {
    /// Enable Debug
    pub mod EDBG {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// Debug mode is disabled.
            pub const DISABLE: u32 = 0;
            /// Debug mode is enabled.
            pub const ENABLE: u32 = 0x01;
        }
    }
    /// Enable Round Robin Channel Arbitration
    pub mod ERCA {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// Round robin channel arbitration is disabled.
            pub const DISABLE: u32 = 0;
            /// Round robin channel arbitration is enabled.
            pub const ENABLE: u32 = 0x01;
        }
    }
    /// Halt After Error
    pub mod HAE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// Normal operation
            pub const DISABLE: u32 = 0;
            /// Any error causes the HALT bit to set. Subsequently, all service requests are ignored until the HALT bit is cleared.
            pub const ENABLE: u32 = 0x01;
        }
    }
    /// Halt DMA Operations
    pub mod HALT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// Normal operation
            pub const DISABLE: u32 = 0;
            /// Stall the start of any new channels. Executing channels are allowed to complete. Channel execution resumes when this bit is cleared.
            pub const ENABLE: u32 = 0x01;
        }
    }
    /// Global Channel Linking Control
    pub mod GCLC {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// Channel linking is disabled for all channels.
            pub const DISABLED: u32 = 0;
            /// Channel linking is available and controlled by each channel's link settings.
            pub const ENABLE: u32 = 0x01;
        }
    }
    /// Global Master ID Replication Control
    pub mod GMRC {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// Master ID replication is disabled for all channels.
            pub const DISABLE: u32 = 0;
            /// Master ID replication is available and is controlled by each channel's CHn_SBR\\[EMI\\] setting.
            pub const ENABLE: u32 = 0x01;
        }
    }
    /// eDMA version
    pub mod VER {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    /// Active channel ID
    pub mod ACTIVE_ID {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    /// DMA Active Status
    pub mod ACTIVE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// eDMA is idle.
            pub const IDLE: u32 = 0;
            /// eDMA is executing a channel.
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
/// Management Page Error Status
pub mod MP_ES {
    /// Destination Bus Error
    pub mod DBE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// No destination bus error
            pub const NO_ERROR: u32 = 0;
            /// The last recorded error was a bus error on a destination write
            pub const ERROR: u32 = 0x01;
        }
    }
    /// Source Bus Error
    pub mod SBE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// No source bus error
            pub const NO_ERROR: u32 = 0;
            /// The last recorded error was a bus error on a source read
            pub const ERROR: u32 = 0x01;
        }
    }
    /// Scatter/Gather Configuration Error
    pub mod SGE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// No scatter/gather configuration error
            pub const NO_ERROR: u32 = 0;
            /// The last recorded error was a configuration error detected in the TCDn_DLASTSGA field. This field is checked at the beginning of a scatter/gather operation after major loop completion if TCDn_CSR\\[ESG\\] is enabled. TCDn_DLASTSGA is not on a 32 byte boundary.
            pub const ERROR: u32 = 0x01;
        }
    }
    /// NBYTES/CITER Configuration Error
    pub mod NCE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// No NBYTES/CITER configuration error
            pub const NO_ERROR: u32 = 0;
            /// The last recorded error was NBYTES equal to zero or a CITER not equal to BITER error
            pub const ERROR: u32 = 0x01;
        }
    }
    /// Destination Offset Error
    pub mod DOE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// No destination offset configuration error
            pub const NO_ERROR: u32 = 0;
            /// The last recorded error was a configuration error detected in the TCDn_DOFF field. TCDn_DOFF is inconsistent with TCDn_ATTR\\[DSIZE\\].
            pub const ERROR: u32 = 0x01;
        }
    }
    /// Destination Address Error
    pub mod DAE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// No destination address configuration error
            pub const NO_ERROR: u32 = 0;
            /// The last recorded error was a configuration error detected in the TCDn_DADDR field. TCDn_DADDR is inconsistent with TCDn_ATTR\\[DSIZE\\].
            pub const ERROR: u32 = 0x01;
        }
    }
    /// Source Offset Error
    pub mod SOE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// No source offset configuration error
            pub const NO_ERROR: u32 = 0;
            /// The last recorded error was a configuration error detected in the TCDn_SOFF field. TCDn_SOFF is inconsistent with TCDn_ATTR\\[SSIZE\\].
            pub const ERROR: u32 = 0x01;
        }
    }
    /// Source Address Error
    pub mod SAE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// No source address configuration error.
            pub const NO_ERROR: u32 = 0;
            /// The last recorded error was a configuration error detected in the TCDn_SADDR field. TCDn_SADDR is inconsistent with TCDn_ATTR\\[SSIZE\\].
            pub const ERROR: u32 = 0x01;
        }
    }
    /// Transfer Canceled
    pub mod ECX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// No canceled transfers
            pub const NO_ERROR: u32 = 0;
            /// The last recorded entry was a canceled transfer by the error cancel transfer input.
            pub const ERROR: u32 = 0x01;
        }
    }
    /// Error Channel Number or Canceled Channel Number
    pub mod ERRCHN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    /// Valid
    pub mod VLD {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// No ERR bits are set.
            pub const NO_ERROR: u32 = 0;
            /// At least one ERR bit is set indicating a valid error exists that has not been cleared.
            pub const ERROR: u32 = 0x01;
        }
    }
}
/// Management Page Interrupt Request Status - Low
pub mod MP_INT_LOW {
    /// Interrupt Request Status for channels 31 - 0
    pub mod INT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
/// Management Page Interrupt Request Status - High
pub mod MP_INT_HIGH {
    /// Interrupt Request Status for channels 63-32
    pub mod INT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
/// Management Page Hardware Request Status - Low
pub mod MP_HRS_LOW {
    /// Hardware Request Status for channels 31 - 0
    pub mod HRS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// A hardware service request for the channel is not present
            pub const IDLE: u32 = 0;
            /// A hardware service request for channel 0 is present
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
/// Management Page Hardware Request Status - High
pub mod MP_HRS_HIGH {
    /// Hardware Request Status for channels 63-32
    pub mod HRS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// A hardware service request for the channel is not present
            pub const IDLE: u32 = 0;
            /// A hardware service request for channel 0 is present
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
/// Channel Arbitration Group
pub mod CH_GRPRI {
    /// Arbitration group per channel.
    pub mod GRPRI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
