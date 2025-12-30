// Generated macro for FAIL (static)
macro_rules! Depcrate_allocateFAIL {
() => {
// Module: crate::allocate
// Provides: {"FAIL"}
// Dependencies: {}
# [cfg (test)] static FAIL : Allocator < 'static > = Allocator { zalloc : zalloc_fail , zfree : zfree_fail , opaque : core :: ptr :: null_mut () , _marker : PhantomData , } ;
};
}
