// Generated macro for impl_65 (impl)
macro_rules! Depcrate_internal_castimpl_65 {
() => {
// Module: crate::internal::cast
// Provides: {"impl_65"}
// Dependencies: {}
impl ValueBag < 'static > { # [doc = " Try capture an owned raw value."] # [doc = ""] # [doc = " This method will return `Some` if the value is a simple primitive"] # [doc = " that can be captured without losing its structure. In other cases"] # [doc = " this method will return `None`."] # [cfg (feature = "owned")] pub fn try_capture_owned < T > (value : & '_ T) -> Option < Self > where T : ? Sized + 'static , { primitive :: from_owned_any (value) } }
};
}
