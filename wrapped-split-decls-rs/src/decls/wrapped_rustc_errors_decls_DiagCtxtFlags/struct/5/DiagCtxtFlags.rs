use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Copy , Clone , Default)] pub struct DiagCtxtFlags { # [doc = " If false, warning-level lints are suppressed."] # [doc = " (rustc: see `--allow warnings` and `--cap-lints`)"] pub can_emit_warnings : bool , # [doc = " If Some, the Nth error-level diagnostic is upgraded to bug-level."] # [doc = " (rustc: see `-Z treat-err-as-bug`)"] pub treat_err_as_bug : Option < NonZero < usize > > , # [doc = " Eagerly emit delayed bugs as errors, so that the compiler debugger may"] # [doc = " see all of the errors being emitted at once."] pub eagerly_emit_delayed_bugs : bool , # [doc = " Show macro backtraces."] # [doc = " (rustc: see `-Z macro-backtrace`)"] pub macro_backtrace : bool , # [doc = " If true, identical diagnostics are reported only once."] pub deduplicate_diagnostics : bool , # [doc = " Track where errors are created. Enabled with `-Ztrack-diagnostics`."] pub track_diagnostics : bool , }
}