macro_rules! deps {
    () => {
        Allocator!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl z_stream { fn configure_allocator (& mut self , alloc : Allocator) { self . zalloc = Some (alloc . zalloc) ; self . zfree = Some (alloc . zfree) ; self . opaque = alloc . opaque ; } # [cfg (feature = "rust-allocator")] pub fn configure_default_rust_allocator (& mut self) { self . configure_allocator (crate :: allocate :: RUST) } # [cfg (feature = "c-allocator")] pub fn configure_default_c_allocator (& mut self) { self . configure_allocator (crate :: allocate :: C) } }
    };
}

impl_48!()