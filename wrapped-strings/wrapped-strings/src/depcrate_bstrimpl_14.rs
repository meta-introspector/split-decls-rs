// Generated macro for impl_14 (impl)
macro_rules! Depcrate_bstrimpl_14 {
() => {
// Module: crate::bstr
// Provides: {"impl_14"}
// Dependencies: {}
impl TryFrom < BSTR > for String { type Error = alloc :: string :: FromUtf16Error ; fn try_from (value : BSTR) -> core :: result :: Result < Self , Self :: Error > { Self :: try_from (& value) } }
};
}
