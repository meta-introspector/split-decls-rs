// Generated macro for impl_257 (impl)
macro_rules! Depcrate_format_description_borrowed_format_itemimpl_257 {
() => {
// Module: crate::format_description::borrowed_format_item
// Provides: {"impl_257"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl fmt :: Debug for BorrowedFormatItem < '_ > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Literal (literal) => f . write_str (& String :: from_utf8_lossy (literal)) , Self :: Component (component) => component . fmt (f) , Self :: Compound (compound) => compound . fmt (f) , Self :: Optional (item) => f . debug_tuple ("Optional") . field (item) . finish () , Self :: First (items) => f . debug_tuple ("First") . field (items) . finish () , } } }
};
}
