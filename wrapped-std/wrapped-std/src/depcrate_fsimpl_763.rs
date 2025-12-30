// Generated macro for impl_763 (impl)
macro_rules! Depcrate_fsimpl_763 {
() => {
// Module: crate::fs
// Provides: {"impl_763"}
// Dependencies: {}
# [stable (feature = "dir_entry_debug" , since = "1.13.0")] impl fmt :: Debug for DirEntry { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("DirEntry") . field (& self . path ()) . finish () } }
};
}
