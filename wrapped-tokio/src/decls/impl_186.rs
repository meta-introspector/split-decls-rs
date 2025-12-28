macro_rules! deps {
    () => {
        AtomicU32!();
        UnsafeCell!();
    };
}

macro_rules! impl_186 {
    () => {
        deps!();
        impl AtomicU32 { pub (crate) const fn new (val : u32) -> AtomicU32 { let inner = UnsafeCell :: new (std :: sync :: atomic :: AtomicU32 :: new (val)) ; AtomicU32 { inner } } # [doc = " Performs an unsynchronized load."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " All mutations must have happened before the unsynchronized load."] # [doc = " Additionally, there must be no concurrent mutations."] pub (crate) unsafe fn unsync_load (& self) -> u32 { unsafe { core :: ptr :: read (self . inner . get () as * const u32) } } }
    };
}

impl_186!();