// Generated macro for impl_2821 (impl)
macro_rules! Depcrate_pathimpl_2821 {
() => {
// Module: crate::path
// Provides: {"impl_2821"}
// Dependencies: {}
# [stable (feature = "path_components_debug" , since = "1.13.0")] impl fmt :: Debug for Components < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { struct DebugHelper < 'a > (& 'a Path) ; impl fmt :: Debug for DebugHelper < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . 0 . components ()) . finish () } } f . debug_tuple ("Components") . field (& DebugHelper (self . as_path ())) . finish () } }
};
}
