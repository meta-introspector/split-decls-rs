macro_rules! deps {
    () => {
        RustStringInner!();
    };
}

macro_rules! LLVMRustStringWriteImpl {
    () => {
        deps!();
        # [doc = " Appends the contents of a byte slice to a [`RustString`]."] # [doc = ""] # [doc = " This function is implemented in `rustc_llvm` so that the C++ code in this"] # [doc = " crate can link to it directly, without an implied link-time dependency on"] # [doc = " `rustc_codegen_llvm`."] # [unsafe (no_mangle)] pub unsafe extern "C" fn LLVMRustStringWriteImpl (buf : & RustString , slice_ptr : * const u8 , slice_len : size_t ,) { let slice = unsafe { slice :: from_raw_parts (slice_ptr , slice_len) } ; RustStringInner :: from_opaque (buf) . bytes . borrow_mut () . extend_from_slice (slice) ; }
    };
}

LLVMRustStringWriteImpl!();