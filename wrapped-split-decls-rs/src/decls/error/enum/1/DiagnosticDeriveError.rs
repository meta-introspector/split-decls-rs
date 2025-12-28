use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug)] pub (crate) enum DiagnosticDeriveError { SynError (SynError) , ErrorHandled , }