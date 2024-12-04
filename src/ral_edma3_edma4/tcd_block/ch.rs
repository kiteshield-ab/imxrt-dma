use ral_registers::RWRegister;

/// CH part of TCD block.
#[repr(C)]
pub struct RegisterBlock {
    /// Channel Control and Status Register
    pub CSR: RWRegister<u32>,
    /// Channel Error Status Register
    pub ES: RWRegister<u32>,
    /// Channel Interrupt Status Register
    pub INT: RWRegister<u32>,
    /// Channel System Bus Register
    pub SBR: RWRegister<u32>,
    /// Channel Priority Register
    pub PRI: RWRegister<u32>,
    /// Channel Multiplexor Configuration
    pub MUX: RWRegister<u32>,
    /// Memory Attributes Register (only on eDMA4, not eDMA3)
    pub MATTR: RWRegister<u16>,
}

/// Channel Control and Status Register
pub mod CSR {
    /// Enable DMA Request
    pub mod ERQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// The DMA hardware request signal for the corresponding channel is disabled.
            pub const DISABLE: u32 = 0;
            /// The DMA hardware request signal for the corresponding channel is enabled.
            pub const ENABLE: u32 = 0x01;
        }
    }
    /// Enable Asynchronous DMA Request
    pub mod EARQ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// Disable asynchronous DMA request for the channel.
            pub const DISABLE: u32 = 0;
            /// Enable asynchronous DMA request for the channel.
            pub const ENABLE: u32 = 0x01;
        }
    }
    /// Enable Error Interrupt
    pub mod EEI {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// The error signal for corresponding channel does not generate an error interrupt
            pub const DISABLE: u32 = 0;
            /// The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const ENABLE: u32 = 0x01;
        }
    }
    /// Swap size
    pub mod SWAP {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// disabled
            pub const DISABLE: u32 = 0;
            /// read with 8-bit swap
            pub const READ_SWAP8: u32 = 0x01;
            /// read with 16-bit swap
            pub const READ_SWAP16: u32 = 0x02;
            /// read with 32-bit swap
            pub const READ_SWAP32: u32 = 0x03;
            /// write with 8-bit swap
            pub const WRITE_SWAP8: u32 = 0x09;
            /// write with 16-bit swap
            pub const WRITE_SWAP16: u32 = 0x0a;
            /// write with 32-bit swap
            pub const WRITE_SWAP32: u32 = 0x0b;
        }
    }
    /// Sign Extension
    pub mod SIGNEXT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// disabled
            pub const DISABLE: u32 = 0;
            /// A non-zero value specifying the sign extend bit position
            pub const ENABLE: u32 = 0x01;
        }
    }
    /// Channel Done
    pub mod DONE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    /// Channel Active
    pub mod ACTIVE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
/// Channel Error Status Register
pub mod ES {
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
            /// The last recorded error was a configuration error detected in the TCDn_NBYTES or TCDn_CITER fields. TCDn_NBYTES is not a multiple of TCDn_ATTR\\[SSIZE\\] and TCDn_ATTR\\[DSIZE\\], or TCDn_CITER\\[CITER\\] is equal to zero, or TCDn_CITER\\[ELINK\\] is not equal to TCDn_BITER\\[ELINK\\]
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
    /// Error In Channel
    pub mod ERR {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// An error in this channel has not occurred
            pub const NO_ERROR: u32 = 0;
            /// An error in this channel has occurred
            pub const ERROR: u32 = 0x01;
        }
    }
}
/// Channel Interrupt Status Register
pub mod INT {
    /// Interrupt Request
    pub mod INT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// The interrupt request for corresponding channel is cleared
            pub const INACTIVE: u32 = 0;
            /// The interrupt request for corresponding channel is active
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
/// Channel System Bus Register
pub mod SBR {
    /// Master ID
    pub mod MID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    /// Instruction/Data Access
    pub mod INSTR {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// Data access for DMA transfers
            pub const DATA: u32 = 0;
            /// Instruction access for DMA transfers
            pub const INSTR: u32 = 0x01;
        }
    }
    /// Security Level
    pub mod SEC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// Nonsecure protection level for DMA transfers
            pub const NONSECURE: u32 = 0;
            /// Secure protection level for DMA transfers
            pub const SECURE: u32 = 0x01;
        }
    }
    /// Privileged Access Level
    pub mod PAL {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// User protection level for DMA transfers
            pub const USER: u32 = 0;
            /// Privileged protection level for DMA transfers
            pub const PRIV: u32 = 0x01;
        }
    }
    /// Enable Master ID replication
    pub mod EMI {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// Master ID replication is disabled
            pub const DISABLE: u32 = 0;
            /// Master ID replication is enabled
            pub const ENABLE: u32 = 0x01;
        }
    }
    /// Attribute Output
    pub mod ATTR {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
/// Channel Priority Register
pub mod PRI {
    /// Arbitration Priority Level
    pub mod APL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    /// Disable Preempt Ability.
    pub mod DPA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// The channel can suspend a lower priority channel.
            pub const DISABLE: u32 = 0;
            /// The channel cannot suspend any other channel, regardless of channel priority.
            pub const ENABLE: u32 = 0x01;
        }
    }
    /// Enable Channel Preemption.
    pub mod ECP {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// The channel cannot be suspended by a higher priority channel's service request.
            pub const DISABLE: u32 = 0;
            /// The channel can be temporarily suspended by the service request of a higher priority channel.
            pub const ENABLE: u32 = 0x01;
        }
    }
}
/// Channel Multiplexor Configuration
pub mod MUX {
    /// Service Request Source
    pub mod SRC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
/// Memory Attributes Register
pub mod MATTR {
    /// Read Cache Attributes
    pub mod RCACHE {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    /// Write Cache Attributes
    pub mod WCACHE {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
