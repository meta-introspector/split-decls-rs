// Generated macro for impl_43 (impl)
macro_rules! Depcrate_arcimpl_43 {
() => {
// Module: crate::arc
// Provides: {"impl_43"}
// Dependencies: {}
impl < T : ? Sized > Clone for Arc < T > { # [inline] fn clone (& self) -> Self { let old_size = self . inner () . count . fetch_add (1 , Relaxed) ; if old_size > MAX_REFCOUNT { abort () ; } unsafe { Arc { p : ptr :: NonNull :: new_unchecked (self . ptr ()) , phantom : PhantomData , } } } }
};
}
