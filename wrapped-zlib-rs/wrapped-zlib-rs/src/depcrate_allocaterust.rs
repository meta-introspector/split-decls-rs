// Generated macro for RUST (static)
macro_rules! Depcrate_allocateRUST {
() => {
// Module: crate::allocate
// Provides: {"RUST"}
// Dependencies: {}
# [cfg (feature = "rust-allocator")] pub static RUST : Allocator < 'static > = Allocator { zalloc : zalloc_rust , zfree : zfree_rust , opaque : core :: ptr :: null_mut () , _marker : PhantomData , } ;
};
}
