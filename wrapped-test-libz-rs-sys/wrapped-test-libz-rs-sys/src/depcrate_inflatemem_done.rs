// Generated macro for mem_done (function)
macro_rules! Depcrate_inflatemem_done {
() => {
// Module: crate::inflate
// Provides: {"mem_done"}
// Dependencies: {}
fn mem_done (stream : & mut libz_rs_sys :: z_stream) { assert ! (! stream . opaque . is_null ()) ; extern "C" { fn free (p : * mut c_void) ; } let mut zone = unsafe { Box :: from_raw (stream . opaque as * mut MemZone) } ; let count = zone . items . len () ; for item in zone . items . drain (..) { unsafe { free (item . ptr) } ; } assert_eq ! ((count , zone . total) , (0 , 0) , "{} bytes in {count} blocks not freed" , zone . total) ; assert_eq ! (zone . not_lifo , 0 , "{} frees not LIFO" , zone . not_lifo) ; assert_eq ! (zone . rogue , 0 , "{} frees not recognized" , zone . rogue) ; stream . opaque = std :: ptr :: null_mut () ; stream . zalloc = None ; stream . zfree = None ; }
};
}
