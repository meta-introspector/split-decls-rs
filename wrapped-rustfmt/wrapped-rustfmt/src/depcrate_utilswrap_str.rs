// Generated macro for wrap_str (function)
macro_rules! Depcrate_utilswrap_str {
() => {
// Module: crate::utils
// Provides: {"wrap_str"}
// Dependencies: {}
pub (crate) fn wrap_str (s : String , max_width : usize , shape : Shape) -> Option < String > { if filtered_str_fits (& s , max_width , shape) { Some (s) } else { None } }
};
}
