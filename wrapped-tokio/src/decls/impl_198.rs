macro_rules! deps {
    () => {
        UnsafeCell!();
        AtomicUsize!();
    };
}

macro_rules! impl_198 {
    () => {
        deps!();
        impl AtomicUsize { pub (crate) const fn new (val : usize) -> AtomicUsize { let inner = UnsafeCell :: new (std :: sync :: atomic :: AtomicUsize :: new (val)) ; AtomicUsize { inner } } # [doc = " Performs an unsynchronized load."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " All mutations must have happened before the unsynchronized load."] # [doc = " Additionally, there must be no concurrent mutations."] pub (crate) unsafe fn unsync_load (& self) -> usize { unsafe { core :: ptr :: read (self . inner . get () as * const usize) } } pub (crate) fn with_mut < R > (& mut self , f : impl FnOnce (& mut usize) -> R) -> R { f (unsafe { (* self . inner . get ()) . get_mut () }) } }
    };
}

impl_198!()