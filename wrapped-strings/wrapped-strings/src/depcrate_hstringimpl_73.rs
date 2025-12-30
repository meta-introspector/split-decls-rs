// Generated macro for impl_73 (impl)
macro_rules! Depcrate_hstringimpl_73 {
() => {
// Module: crate::hstring
// Provides: {"impl_73"}
// Dependencies: {}
impl TryFrom < HSTRING > for String { type Error = alloc :: string :: FromUtf16Error ; fn try_from (hstring : HSTRING) -> core :: result :: Result < Self , Self :: Error > { Self :: try_from (& hstring) } }
};
}
