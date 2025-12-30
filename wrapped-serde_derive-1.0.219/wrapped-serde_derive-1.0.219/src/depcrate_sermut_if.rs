// Generated macro for mut_if (function)
macro_rules! Depcrate_sermut_if {
() => {
// Module: crate::ser
// Provides: {"mut_if"}
// Dependencies: {}
fn mut_if (is_mut : bool) -> Option < TokenStream > { if is_mut { Some (quote ! (mut)) } else { None } }
};
}
