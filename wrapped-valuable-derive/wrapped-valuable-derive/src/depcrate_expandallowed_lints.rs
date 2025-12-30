// Generated macro for allowed_lints (function)
macro_rules! Depcrate_expandallowed_lints {
() => {
// Module: crate::expand
// Provides: {"allowed_lints"}
// Dependencies: {}
fn allowed_lints () -> TokenStream { quote ! { # [allow (non_upper_case_globals)] # [allow (clippy :: unknown_clippy_lints)] # [allow (clippy :: used_underscore_binding)] # [allow (clippy :: indexing_slicing)] } }
};
}
