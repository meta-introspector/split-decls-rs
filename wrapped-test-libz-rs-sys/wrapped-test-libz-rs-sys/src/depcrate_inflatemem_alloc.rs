// Generated macro for mem_alloc (function)
macro_rules! Depcrate_inflatemem_alloc {
() => {
// Module: crate::inflate
// Provides: {"mem_alloc"}
// Dependencies: {}
unsafe extern "C" fn mem_alloc (mem : * mut c_void , count : u32 , size : u32) -> * mut c_void { let count = count as usize ; let size = size as usize ; let len = count * size ; if mem . is_null () { return std :: ptr :: null_mut () ; } let mut zone = ManuallyDrop :: new (unsafe { Box :: from_raw (mem as * mut MemZone) }) ; if zone . limit != 0 && zone . total + len > zone . limit { return std :: ptr :: null_mut () ; } extern "C" { fn malloc (size : usize) -> * mut c_void ; } let ptr = unsafe { malloc (len) } ; if ptr . is_null () { return std :: ptr :: null_mut () ; } unsafe { std :: ptr :: write_bytes (ptr , 0xa5 , len) } ; let item = MemItem { ptr , size : len } ; zone . total += item . size ; if zone . total > zone . highwater { zone . highwater = zone . total ; } zone . items . push (item) ; ptr }
};
}
