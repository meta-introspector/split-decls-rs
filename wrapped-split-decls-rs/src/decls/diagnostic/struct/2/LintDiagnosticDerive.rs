use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " The central struct for constructing the `decorate_lint` method from an annotated struct."] pub (crate) struct LintDiagnosticDerive < 'a > { structure : Structure < 'a > , }
}