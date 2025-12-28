use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub (super) struct SubdiagnosticVariant { pub (super) kind : SubdiagnosticKind , pub (super) slug : Option < Path > , pub (super) no_span : bool , }