use ral_registers::{RORegister, RWRegister};

/// DMA MP
#[repr(C)]
pub struct RegisterBlock {
    /// Management Page Control
    pub MP_CSR: RWRegister<u32>,
    /// Management Page Error Status
    pub MP_ES: RORegister<u32>,
    /// Management Page Interrupt Request Status
    pub MP_INT: RORegister<u32>,
    /// Management Page Hardware Request Status
    pub MP_HRS: RORegister<u32>,
    _reserved0: [u8; 0xf0], // 0xe8 for 64 channels
    /// Channel Arbitration Group
    pub CH_GRPRI: [RWRegister<u32>; 32usize], // 64
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
            /// Debug mode disabled
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
            /// Round-robin channel arbitration disabled
            pub const DISABLE: u32 = 0;
            /// Round-robin channel arbitration enabled
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
            pub const NORMAL_OPERATION: u32 = 0;
            /// Any error causes the HALT field to be set to 1
            pub const HALT: u32 = 0x01;
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
            pub const NORMAL_OPERATION: u32 = 0;
            /// Stall the start of any new channels
            pub const STALL: u32 = 0x01;
        }
    }
    /// Global Channel Linking Control
    pub mod GCLC {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// Channel linking disabled for all channels
            pub const DISABLE: u32 = 0;
            /// Channel linking available and controlled by each channel's link settings
            pub const AVAILABLE: u32 = 0x01;
        }
    }
    /// Global Master ID Replication Control
    pub mod GMRC {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// Master ID replication disabled for all channels
            pub const DISABLE: u32 = 0;
            /// Master ID replication available and controlled by each channel's CHn_SBR\\[EMI\\] setting
            pub const AVAILABLE: u32 = 0x01;
        }
    }
    /// Cancel Transfer With Error
    pub mod ECX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// Normal operation
            pub const NORMAL_OPERATION: u32 = 0;
            /// Cancel the remaining data transfer
            pub const CANCEL: u32 = 0x01;
        }
    }
    /// Cancel Transfer
    pub mod CX {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// Normal operation
            pub const NORMAL_OPERATION: u32 = 0;
            /// Cancel the remaining data transfer
            pub const DATA_TRANSFER_CANCEL: u32 = 0x01;
        }
    }
    /// Active Channel ID
    pub mod ACTIVE_ID {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
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
            /// eDMA is idle
            pub const IDLE: u32 = 0;
            /// eDMA is executing a channel
            pub const EXECUTION: u32 = 0x01;
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
            /// Last recorded error was a bus error on a destination write
            pub const BUS_ERROR: u32 = 0x01;
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
            /// Last recorded error was a bus error on a source read
            pub const BUS_ERROR: u32 = 0x01;
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
            /// Last recorded error was a configuration error detected in the TCDn_DLAST_SGA field
            pub const CONFIGURATION_ERROR: u32 = 0x01;
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
            pub const CONFIGURATION_ERROR: u32 = 0x01;
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
            /// Last recorded error was a configuration error detected in the TCDn_DOFF field
            pub const CONFIGURATION_ERROR: u32 = 0x01;
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
            /// Last recorded error was a configuration error detected in the TCDn_DADDR field
            pub const CONFIGURATION_ERROR: u32 = 0x01;
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
            /// Last recorded error was a configuration error detected in the TCDn_SOFF field
            pub const CONFIGURATION_ERROR: u32 = 0x01;
        }
    }
    /// Source Address Error
    pub mod SAE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// No source address configuration error
            pub const NO_ERROR: u32 = 0;
            /// Last recorded error was a configuration error detected in the TCDn_SADDR field
            pub const CONFIGURATION_ERROR: u32 = 0x01;
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
            pub const NO_CANCELED_TRANSFERS: u32 = 0;
            /// Last recorded entry was a canceled transfer by the error cancel transfer input
            pub const CANCELED_TRANSFER: u32 = 0x01;
        }
    }
    /// Error Channel Number or Canceled Channel Number
    pub mod ERRCHN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
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
            /// No CHn_ES\\[ERR\\] fields are set to 1
            pub const NO_FIELD_SET_ONE: u32 = 0;
            /// At least one CHn_ES\\[ERR\\] field is set to 1, indicating a valid error exists that software has not cleared
            pub const ATLEAST_ONE_FIELD: u32 = 0x01;
        }
    }
}
/// Management Page Interrupt Request Status
pub mod MP_INT {
    /// Interrupt Request Status
    pub mod INT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
/// Management Page Hardware Request Status
pub mod MP_HRS {
    /// Hardware Request Status
    pub mod HRS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
/// Channel Arbitration Group
pub mod CH_GRPRI {
    /// Arbitration Group For Channel n
    pub mod GRPRI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
