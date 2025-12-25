use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Appends the contents of a byte slice to a [`RustString`].
///
/// This function is implemented in `rustc_llvm` so that the C++ code in this
/// crate can link to it directly, without an implied link-time dependency on
/// `rustc_codegen_llvm`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LLVMRustStringWriteImpl(
    buf: &RustString,
    slice_ptr: *const u8,
    slice_len: size_t,
) {
    let slice = unsafe { slice::from_raw_parts(slice_ptr, slice_len) };
    RustStringInner::from_opaque(buf).bytes.borrow_mut().extend_from_slice(slice);
}
