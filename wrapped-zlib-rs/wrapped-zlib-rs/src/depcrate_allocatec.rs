// Generated macro for C (static)
macro_rules! Depcrate_allocateC {
() => {
// Module: crate::allocate
// Provides: {"C"}
// Dependencies: {}
# [cfg (feature = "c-allocator")] pub static C : Allocator < 'static > = Allocator { zalloc : zalloc_c , zfree : zfree_c , opaque : core :: ptr :: null_mut () , _marker : PhantomData , } ;
};
}
