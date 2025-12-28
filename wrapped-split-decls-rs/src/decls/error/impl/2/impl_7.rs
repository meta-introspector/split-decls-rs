use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < SynError > for DiagnosticDeriveError { fn from (e : SynError) -> Self { DiagnosticDeriveError :: SynError (e) } }
}