// Generated macro for impl_44 (impl)
macro_rules! Depcrate_detect_cacheimpl_44 {
() => {
// Module: crate::detect::cache
// Provides: {"impl_44"}
// Dependencies: {}
impl Cache { const CAPACITY : u32 = (core :: mem :: size_of :: < usize > () * 8 - 1) as u32 ; const MASK : usize = (1 << Cache :: CAPACITY) - 1 ; const INITIALIZED_BIT : usize = 1usize << Cache :: CAPACITY ; # [doc = " Creates an uninitialized cache."] # [allow (clippy :: declare_interior_mutable_const)] const fn uninitialized () -> Self { Cache (AtomicUsize :: new (0)) } # [doc = " Is the `bit` in the cache set? Returns `None` if the cache has not been initialized."] # [inline] pub (crate) fn test (& self , bit : u32) -> Option < bool > { let cached = self . 0 . load (Ordering :: Relaxed) ; if cached == 0 { None } else { Some (test_bit (cached as u128 , bit)) } } # [doc = " Initializes the cache."] # [inline] fn initialize (& self , value : usize) -> usize { debug_assert_eq ! ((value & ! Cache :: MASK) , 0) ; self . 0 . store (value | Cache :: INITIALIZED_BIT , Ordering :: Relaxed) ; value } }
};
}
