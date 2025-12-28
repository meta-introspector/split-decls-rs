use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl DiagnosticDeriveError { pub (crate) fn to_compile_error (self) -> TokenStream { match self { DiagnosticDeriveError :: SynError (e) => e . to_compile_error () , DiagnosticDeriveError :: ErrorHandled => { quote ! { { unreachable ! () ; } } } } } }
}