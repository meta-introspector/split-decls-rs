// Generated macro for copy_unchecked (function)
macro_rules! Depcrate_utilcopy_unchecked {
() => {
// Module: crate::util
// Provides: {"copy_unchecked"}
// Dependencies: {}
# [doc = " Copies `src` into the prefix of `dst`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The caller guarantees that `src.len() <= dst.len()`."] # [inline (always)] pub (crate) unsafe fn copy_unchecked (src : & [u8] , dst : & mut [u8]) { debug_assert ! (src . len () <= dst . len ()) ; unsafe { core :: ptr :: copy_nonoverlapping (src . as_ptr () , dst . as_mut_ptr () , src . len ()) ; } ; }
};
}
