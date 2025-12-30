// Generated macro for Allocator (struct)
macro_rules! Depcrate_allocateAllocator {
() => {
// Module: crate::allocate
// Provides: {"Allocator"}
// Dependencies: {}
# [derive (Clone , Copy)] # [repr (C)] pub struct Allocator < 'a > { pub zalloc : crate :: c_api :: alloc_func , pub zfree : crate :: c_api :: free_func , pub opaque : crate :: c_api :: voidpf , pub _marker : PhantomData < & 'a () > , }
};
}
