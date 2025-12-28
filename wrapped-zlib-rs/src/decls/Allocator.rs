macro_rules! Allocator {
    () => {
        # [derive (Clone , Copy)] # [repr (C)] pub struct Allocator < 'a > { pub zalloc : crate :: c_api :: alloc_func , pub zfree : crate :: c_api :: free_func , pub opaque : crate :: c_api :: voidpf , pub _marker : PhantomData < & 'a () > , }
    };
}

Allocator!()