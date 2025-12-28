macro_rules! deps {
    () => {
        AtomicU16!();
        UnsafeCell!();
    };
}

macro_rules! impl_177 {
    () => {
        deps!();
        impl AtomicU16 { pub (crate) const fn new (val : u16) -> AtomicU16 { let inner = UnsafeCell :: new (std :: sync :: atomic :: AtomicU16 :: new (val)) ; AtomicU16 { inner } } # [doc = " Performs an unsynchronized load."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " All mutations must have happened before the unsynchronized load."] # [doc = " Additionally, there must be no concurrent mutations."] pub (crate) unsafe fn unsync_load (& self) -> u16 { unsafe { core :: ptr :: read (self . inner . get () as * const u16) } } }
    };
}

impl_177!();