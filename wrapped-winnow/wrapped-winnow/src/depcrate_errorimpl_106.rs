// Generated macro for impl_106 (impl)
macro_rules! Depcrate_errorimpl_106 {
() => {
// Module: crate::error
// Provides: {"impl_106"}
// Dependencies: {}
impl < C : core :: cmp :: PartialEq > core :: cmp :: PartialEq for ContextError < C > { fn eq (& self , other : & Self) -> bool { # [cfg (feature = "alloc")] { if self . context != other . context { return false ; } } # [cfg (feature = "std")] { if self . cause . as_ref () . map (ToString :: to_string) != other . cause . as_ref () . map (ToString :: to_string) { return false ; } } true } }
};
}
