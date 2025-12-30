// Generated macro for memchr (function)
macro_rules! Depcrate_util_memchrmemchr {
() => {
// Module: crate::util::memchr
// Provides: {"memchr"}
// Dependencies: {}
# [cfg (all (unix , feature = "libc"))] pub (crate) fn memchr (needle : u8 , haystack : & [u8]) -> Option < usize > { let start = haystack . as_ptr () ; let ptr = unsafe { libc :: memchr (start . cast () , needle as _ , haystack . len ()) } ; if ptr . is_null () { None } else { Some (ptr as usize - start as usize) } }
};
}
