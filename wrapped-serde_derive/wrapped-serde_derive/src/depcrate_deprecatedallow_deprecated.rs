// Generated macro for allow_deprecated (function)
macro_rules! Depcrate_deprecatedallow_deprecated {
() => {
// Module: crate::deprecated
// Provides: {"allow_deprecated"}
// Dependencies: {}
pub fn allow_deprecated (input : & syn :: DeriveInput) -> Option < TokenStream > { if should_allow_deprecated (input) { Some (quote ! { # [allow (deprecated)] }) } else { None } }
};
}
