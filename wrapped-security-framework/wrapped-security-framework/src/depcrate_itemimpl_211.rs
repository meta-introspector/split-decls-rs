// Generated macro for impl_211 (impl)
macro_rules! Depcrate_itemimpl_211 {
() => {
// Module: crate::item
// Provides: {"impl_211"}
// Dependencies: {}
impl Limit { # [inline] fn to_value (self) -> CFType { match self { Self :: All => unsafe { CFString :: wrap_under_get_rule (kSecMatchLimitAll) . into_CFType () } , Self :: Max (l) => CFNumber :: from (l) . into_CFType () , } } }
};
}
