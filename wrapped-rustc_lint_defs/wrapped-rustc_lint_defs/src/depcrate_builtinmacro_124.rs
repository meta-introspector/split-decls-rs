// Generated macro for macro_124 (macro)
macro_rules! Depcrate_builtinmacro_124 {
() => {
// Module: crate::builtin
// Provides: {"macro_124"}
// Dependencies: {}
declare_lint ! { # [doc = " The `named_arguments_used_positionally` lint detects cases where named arguments are only"] # [doc = " used positionally in format strings. This usage is valid but potentially very confusing."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,compile_fail"] # [doc = " #![deny(named_arguments_used_positionally)]"] # [doc = " fn main() {"] # [doc = "     let _x = 5;"] # [doc = "     println!(\"{}\", _x = 1); // Prints 1, will trigger lint"] # [doc = ""] # [doc = "     println!(\"{}\", _x); // Prints 5, no lint emitted"] # [doc = "     println!(\"{_x}\", _x = _x); // Prints 5, no lint emitted"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Rust formatting strings can refer to named arguments by their position, but this usage is"] # [doc = " potentially confusing. In particular, readers can incorrectly assume that the declaration"] # [doc = " of named arguments is an assignment (which would produce the unit type)."] # [doc = " For backwards compatibility, this is not a hard error."] pub NAMED_ARGUMENTS_USED_POSITIONALLY , Warn , "named arguments in format used positionally" }
};
}
