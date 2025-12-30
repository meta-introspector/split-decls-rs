// Generated macro for impl_46 (impl)
macro_rules! Depcrate_winimpl_46 {
() => {
// Module: crate::win
// Provides: {"impl_46"}
// Dependencies: {}
impl IntoRawHandle for crate :: Handle { fn into_raw_handle (self) -> RawHandle { match self . 0 . kind { HandleKind :: Owned (h) => h . into_raw_handle () , HandleKind :: Borrowed (h) => h . as_raw_handle () , } } }
};
}
