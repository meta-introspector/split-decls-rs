use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Debug)] pub (crate) enum DiagnosticDeriveError { SynError (SynError) , ErrorHandled , }
}