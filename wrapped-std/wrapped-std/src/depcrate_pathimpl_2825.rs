// Generated macro for impl_2825 (impl)
macro_rules! Depcrate_pathimpl_2825 {
() => {
// Module: crate::path
// Provides: {"impl_2825"}
// Dependencies: {}
# [stable (feature = "path_iter_debug" , since = "1.13.0")] impl fmt :: Debug for Iter < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { struct DebugHelper < 'a > (& 'a Path) ; impl fmt :: Debug for DebugHelper < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . 0 . iter ()) . finish () } } f . debug_tuple ("Iter") . field (& DebugHelper (self . as_path ())) . finish () } }
};
}
