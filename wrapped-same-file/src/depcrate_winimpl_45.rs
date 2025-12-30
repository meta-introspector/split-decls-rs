// Generated macro for impl_45 (impl)
macro_rules! Depcrate_winimpl_45 {
() => {
// Module: crate::win
// Provides: {"impl_45"}
// Dependencies: {}
impl AsRawHandle for crate :: Handle { fn as_raw_handle (& self) -> RawHandle { match self . 0 . kind { HandleKind :: Owned (ref h) => h . as_raw_handle () , HandleKind :: Borrowed (ref h) => h . as_raw_handle () , } } }
};
}
