// Generated macro for impl_13 (impl)
macro_rules! Depcrate_bstrimpl_13 {
() => {
// Module: crate::bstr
// Provides: {"impl_13"}
// Dependencies: {}
impl TryFrom < & BSTR > for String { type Error = alloc :: string :: FromUtf16Error ; fn try_from (value : & BSTR) -> core :: result :: Result < Self , Self :: Error > { Self :: from_utf16 (value) } }
};
}
