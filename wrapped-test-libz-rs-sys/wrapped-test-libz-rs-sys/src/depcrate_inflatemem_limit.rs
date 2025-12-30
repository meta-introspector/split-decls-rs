// Generated macro for mem_limit (function)
macro_rules! Depcrate_inflatemem_limit {
() => {
// Module: crate::inflate
// Provides: {"mem_limit"}
// Dependencies: {}
fn mem_limit (stream : & mut libz_rs_sys :: z_stream , limit : usize) { assert ! (! stream . opaque . is_null ()) ; let mut zone = ManuallyDrop :: new (unsafe { Box :: from_raw (stream . opaque as * mut MemZone) }) ; zone . limit = limit ; }
};
}
