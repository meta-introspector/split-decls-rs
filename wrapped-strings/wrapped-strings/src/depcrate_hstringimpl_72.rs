// Generated macro for impl_72 (impl)
macro_rules! Depcrate_hstringimpl_72 {
() => {
// Module: crate::hstring
// Provides: {"impl_72"}
// Dependencies: {}
impl TryFrom < & HSTRING > for String { type Error = alloc :: string :: FromUtf16Error ; fn try_from (hstring : & HSTRING) -> core :: result :: Result < Self , Self :: Error > { Self :: from_utf16 (hstring) } }
};
}
