use crate::define_bitflags;

define_bitflags!(
    pub bitflag OpenFlags: u64 {
        ReadOnly = 0x0,
        WriteOnly = 0x1,
        ReadWrite = 0x2,
        AccessMode = 0x3,
        NonBlocking = 0x4,
        SharedLock = 0x10,
        Create = 0x40,
        Exclusive = 0x80,
        NoControlTTY = 0x160,
        Truncate = 0x200,
        Append = 0x400,
        NonBlock = 0x800,
        Dsync = 0x1600,
        Async = 0x2000,
        DirectoryOnly = 0x4000,
        NoFollowSymlink = 0x8000,
        NoAccessTime = 0x40_000,
        CloseOnExec = 0x80_000,
        Sync = 0x101_000,
        Path = 0x200_000,
        TempFile = 0x404_000,
    }
);
