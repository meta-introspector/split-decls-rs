// Generated macro for impl_308 (impl)
macro_rules! Depcrate_format_description_owned_format_itemimpl_308 {
() => {
// Module: crate::format_description::owned_format_item
// Provides: {"impl_308"}
// Dependencies: {}
impl fmt :: Debug for OwnedFormatItem { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Literal (literal) => f . write_str (& String :: from_utf8_lossy (literal)) , Self :: Component (component) => component . fmt (f) , Self :: Compound (compound) => compound . fmt (f) , Self :: Optional (item) => f . debug_tuple ("Optional") . field (item) . finish () , Self :: First (items) => f . debug_tuple ("First") . field (items) . finish () , } } }
};
}
