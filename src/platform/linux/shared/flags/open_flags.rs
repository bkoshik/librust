use crate::define_bitflags;

define_bitflags!(
    pub bitflag OpenFlags: u64 {
        ReadOnly = 0x0,
        WriteOnly = 0x1,
        ReadWrite = 0x2,
        Append = 0x400,
        Create = 0x40,
        Exclusive = 0x80,
        NoControllingTTY = 0x160,
        Truncate = 0x200,
        NonBlock = 0x800,
        DSync = 0x1600,
        Directory = 0x4000,
        NoFollowLink = 0x8000,
        Direct = 0x10_000,
        NoATime = 0x40_000,
        CloseOnExec = 0x80_000,
        Sync = 0x101_000,
        RSync = 0x101_000,
        FSync = 0x101_000,
        Path = 0x200_000,
        TempFile = 0x404_000,
    }
);
