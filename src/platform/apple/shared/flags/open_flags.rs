use crate::define_bitflags;

define_bitflags!(
    pub bitflag OpenFlags: u64 {
        ReadOnly = 0x0,
        WriteOnly = 0x1,
        ReadWrite = 0x2,
        AccessMode = 0x3,
        NonBlocking = 0x4,
        Append = 0x8,
        SharedLock = 0x10,
        ExclusiveLock = 0x20,
        Async = 0x40,
        Sync = 0x80,
        NoFollowSymlink = 0x100,
        Create = 0x200,
        Truncate = 0x400,
        Exclusive = 0x800,
        OnlyEvents = 0x8000,
        NoControlTTY = 0x20_000,
        DirectoryOnly = 0x100_000,
        AllowSymlink = 0x200_000,
        SyncData = 0x400_000,
        CloseOnExec = 0x1_000_000,
        NoFollowAny = 0x20_000_000,
        ExecuteOnly = 0x40_000_000,
        SearchOnly = 0x41_000_000,
    }
);
