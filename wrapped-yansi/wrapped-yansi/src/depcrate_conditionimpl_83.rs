// Generated macro for impl_83 (impl)
macro_rules! Depcrate_conditionimpl_83 {
() => {
// Module: crate::condition
// Provides: {"impl_83"}
// Dependencies: {}
# [allow (unused)] impl CachedBool { const TRUE : u8 = 1 ; const UNINIT : u8 = 2 ; const INITING : u8 = 3 ; pub const fn new () -> Self { CachedBool (AtomicU8 :: new (Self :: UNINIT)) } pub fn get_or_init (& self , f : impl FnOnce () -> bool) -> bool { use core :: sync :: atomic :: Ordering :: * ; match self . 0 . compare_exchange (Self :: UNINIT , Self :: INITING , AcqRel , Relaxed) { Ok (_) => { let new_value = f () ; self . 0 . store (new_value as u8 , Release) ; new_value } Err (Self :: INITING) => { let mut value ; while { value = self . 0 . load (Acquire) ; value } == Self :: INITING { # [cfg (feature = "std")] std :: thread :: yield_now () ; } value == Self :: TRUE } , Err (value) => value == Self :: TRUE , } } }
};
}
