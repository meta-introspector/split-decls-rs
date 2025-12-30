// Generated macro for macro_47 (macro)
macro_rules! Depcrate_builtinmacro_47 {
() => {
// Module: crate::builtin
// Provides: {"macro_47"}
// Dependencies: {}
declare_lint ! { # [doc = " The `unknown_crate_types` lint detects an unknown crate type found in"] # [doc = " a [`crate_type` attribute]."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,compile_fail"] # [doc = " #![crate_type=\"lol\"]"] # [doc = " fn main() {}"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " An unknown value give to the `crate_type` attribute is almost"] # [doc = " certainly a mistake."] # [doc = ""] # [doc = " [`crate_type` attribute]: https://doc.rust-lang.org/reference/linkage.html"] pub UNKNOWN_CRATE_TYPES , Deny , "unknown crate type found in `#[crate_type]` directive" , crate_level_only }
};
}
