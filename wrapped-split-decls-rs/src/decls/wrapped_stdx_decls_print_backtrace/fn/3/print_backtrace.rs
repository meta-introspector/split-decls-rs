use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Prints backtrace to stderr, useful for debugging."] # [expect (clippy :: print_stderr , reason = "only visible to developers")] pub fn print_backtrace () { # [cfg (feature = "backtrace")] eprintln ! ("{:?}" , backtrace :: Backtrace :: new ()) ; # [cfg (not (feature = "backtrace"))] eprintln ! (r#"Enable the backtrace feature.
Uncomment `default = [ "backtrace" ]` in `crates/stdx/Cargo.toml`.
"#) ; }