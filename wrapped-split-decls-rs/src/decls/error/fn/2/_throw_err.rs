use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Helper function for use with `throw_*` macros - constraints `$f` to an `impl FnOnce`."] pub (crate) fn _throw_err (diag : Diagnostic , f : impl FnOnce (Diagnostic) -> Diagnostic ,) -> DiagnosticDeriveError { f (diag) . emit () ; DiagnosticDeriveError :: ErrorHandled }