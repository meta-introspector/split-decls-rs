use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " The central struct for constructing the `into_diag` method from an annotated struct."] pub (crate) struct DiagnosticDerive < 'a > { structure : Structure < 'a > , }
}