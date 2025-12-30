// Generated macro for helpers (module)
macro_rules! Depcratehelpers {
() => {
// Module: crate
// Provides: {"helpers"}
// Dependencies: {}
# [doc = " Re-export some helper fns useful to write filters/fns/tests"] pub mod helpers { # [doc = " Functions helping writing tests"] pub mod tests { pub use crate :: builtins :: testers :: { extract_string , number_args_allowed , value_defined } ; } }
};
}
