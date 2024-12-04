use ral_registers::RWRegister;

/// TCD part of TCD block.
#[repr(C)]
pub struct RegisterBlock {
    /// TCD Source Address Register
    pub SADDR: RWRegister<u32>,
    /// TCD Signed Source Address Offset Register
    pub SOFF: RWRegister<i16>,
    /// TCD Transfer Attributes Register
    pub ATTR: RWRegister<u16>,
    /// TCD Transfer Size without Minor Loop Offsets Register
    pub NBYTES_MLOFFNO: RWRegister<u32>,
    /// TCD Last Source Address Adjustment / Store DADDR Address Register
    pub SLAST_SDA: RWRegister<u32>,
    /// TCD Destination Address Register
    pub DADDR: RWRegister<u32>,
    /// TCD Signed Destination Address Offset Register
    pub DOFF: RWRegister<i16>,
    /// TCD Current Major Loop Count (Minor Loop Channel Linking Disabled) Register
    pub CITER_ELINKNO: RWRegister<u16>,
    /// TCD Last Destination Address Adjustment / Scatter Gather Address Register
    pub DLAST_SGA: RWRegister<u32>,
    /// TCD Control and Status Register
    pub CSR: RWRegister<u16>,
    /// TCD Beginning Major Loop Count (Minor Loop Channel Linking Disabled) Register
    pub BITER_ELINKNO: RWRegister<u16>,
}

/// TCD Source Address Register
pub mod SADDR {
    /// Source Address
    pub mod SADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
/// TCD Signed Source Address Offset Register
pub mod SOFF {
    /// Source address signed offset
    pub mod SOFF {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
/// TCD Transfer Attributes Register
pub mod ATTR {
    /// Destination data transfer size
    pub mod DSIZE {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// 8-bit
            pub const BIT8: u16 = 0;
            /// 16-bit
            pub const BIT16: u16 = 0x01;
            /// 32-bit
            pub const BIT32: u16 = 0x02;
            /// 64-bit
            pub const BIT64: u16 = 0x03;
            /// 16-byte
            pub const BYTE16: u16 = 0x04;
            /// 32-byte
            pub const BYTE32: u16 = 0x05;
            /// 64-byte
            pub const BYTE64: u16 = 0x06;
            /// 128-byte
            pub const BYTE128: u16 = 0x07;
        }
    }
    /// Destination address modulo
    pub mod DMOD {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    /// Source data transfer size
    pub mod SSIZE {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// 8-bit
            pub const BIT8: u16 = 0;
            /// 16-bit
            pub const BIT16: u16 = 0x01;
            /// 32-bit
            pub const BIT32: u16 = 0x02;
            /// 64-bit
            pub const BIT64: u16 = 0x03;
            /// 16-byte
            pub const BYTE16: u16 = 0x04;
            /// 32-byte
            pub const BYTE32: u16 = 0x05;
            /// 64-byte
            pub const BYTE64: u16 = 0x06;
            /// 128-byte
            pub const BYTE128: u16 = 0x07;
        }
    }
    /// Source address modulo
    pub mod SMOD {
        pub const offset: u16 = 11;
        pub const mask: u16 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// Source address modulo feature is disabled
            pub const DISABLE: u16 = 0;
            /// Source address modulo feature is enabled for any non-zero value \\[1-31\\]
            pub const ENABLE: u16 = 0x01;
        }
    }
}
/// TCD Transfer Size without Minor Loop Offsets Register
pub mod NBYTES_MLOFFNO {
    /// Number of Bytes to transfer per service request
    pub mod NBYTES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    /// Destination Minor Loop Offset Enable
    pub mod DMLOE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// The minor loop offset is not applied to the DADDR
            pub const DISABLE: u32 = 0;
            /// The minor loop offset is applied to the DADDR
            pub const ENABLE: u32 = 0x01;
        }
    }
    /// Source Minor Loop Offset Enable
    pub mod SMLOE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// The minor loop offset is not applied to the SADDR
            pub const DISABLE: u32 = 0;
            /// The minor loop offset is applied to the SADDR
            pub const ENABLE: u32 = 0x01;
        }
    }
}
/// TCD Last Source Address Adjustment / Store DADDR Address Register
pub mod SLAST_SDA {
    /// Last Source Address Adjustment / Store DADDR Address
    pub mod SLAST_SDA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
/// TCD Destination Address Register
pub mod DADDR {
    /// Destination Address
    pub mod DADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
/// TCD Signed Destination Address Offset Register
pub mod DOFF {
    /// Destination Address Signed Offset
    pub mod DOFF {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
/// TCD Current Major Loop Count (Minor Loop Channel Linking Disabled) Register
pub mod CITER_ELINKNO {
    /// Current Major Iteration Count
    pub mod CITER {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x7fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    /// Enable channel-to-channel linking on minor-loop complete
    pub mod ELINK {
        pub const offset: u16 = 15;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// The channel-to-channel linking is disabled
            pub const DISABLE: u16 = 0;
            /// The channel-to-channel linking is enabled
            pub const ENABLE: u16 = 0x01;
        }
    }
}
/// TCD Last Destination Address Adjustment / Scatter Gather Address Register
pub mod DLAST_SGA {
    /// Final Destination Address Adjustment / Scatter Gather Address
    pub mod DLAST_SGA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
/// TCD Control and Status Register
pub mod CSR {
    /// Channel Start
    pub mod START {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// The channel is not explicitly started.
            pub const NO_START: u16 = 0;
            /// The channel is explicitly started via a software initiated service request.
            pub const START: u16 = 0x01;
        }
    }
    /// Enable an interrupt when major iteration count completes.
    pub mod INTMAJOR {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// The end-of-major loop interrupt is disabled.
            pub const DISABLE: u16 = 0;
            /// The end-of-major loop interrupt is enabled.
            pub const ENABLE: u16 = 0x01;
        }
    }
    /// Enable an interrupt when major counter is half complete.
    pub mod INTHALF {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// The half-point interrupt is disabled.
            pub const DISABLE: u16 = 0;
            /// The half-point interrupt is enabled.
            pub const ENABLE: u16 = 0x01;
        }
    }
    /// Disable request
    pub mod DREQ {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// No operation
            pub const DISABLE: u16 = 0;
            /// Clear the ERQ bit upon major loop completion, thus disabling hardware service requests.
            pub const ENABLE: u16 = 0x01;
        }
    }
    /// Enable Scatter/Gather processing
    pub mod ESG {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// The current channel's TCD is normal format.
            pub const DISABLE: u16 = 0;
            /// The current channel's TCD specifies a scatter gather format. The DLASTSGA field provides a memory pointer to the next TCD to be loaded into this channel after the major loop completes its execution.
            pub const ENABLE: u16 = 0x01;
        }
    }
    /// Enable channel-to-channel linking on major loop complete
    pub mod MAJORELINK {
        pub const offset: u16 = 5;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// The channel-to-channel linking is disabled.
            pub const DISABLE: u16 = 0;
            /// The channel-to-channel linking is enabled.
            pub const ENABLE: u16 = 0x01;
        }
    }
    /// Enable end-of-packet processing
    pub mod EEOP {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// The end-of-packet operation is disabled.
            pub const DISABLE: u16 = 0;
            /// The end-of-packet hardware input signal is enabled.
            pub const ENABLE: u16 = 0x01;
        }
    }
    /// Enable store destination address
    pub mod ESDA {
        pub const offset: u16 = 7;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// The store destination address to system memory operation is disabled.
            pub const DISABLE: u16 = 0;
            /// The store destination address to system memory operation is enabled.
            pub const ENABLE: u16 = 0x01;
        }
    }
    /// Major loop link channel number
    pub mod MAJORLINKCH {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    /// Transfer Mode Control
    pub mod TMC {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// Read/Write
            pub const NORMAL: u16 = 0;
            /// Read Only
            pub const READ_ONLY: u16 = 0x01;
            /// Write Only
            pub const WRITE_ONLY: u16 = 0x02;
        }
    }
}
/// TCD Beginning Major Loop Count (Minor Loop Channel Linking Disabled) Register
pub mod BITER_ELINKNO {
    /// Starting Major Iteration Count
    pub mod BITER {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x7fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    /// Enables channel-to-channel linking on minor loop complete
    pub mod ELINK {
        pub const offset: u16 = 15;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            /// The channel-to-channel linking is disabled
            pub const DISABLE: u16 = 0;
            /// The channel-to-channel linking is enabled
            pub const ENABLE: u16 = 0x01;
        }
    }
}
