use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl FutureIncompatibleInfo { pub const fn default_fields_for_macro () -> Self { FutureIncompatibleInfo { reference : "" , reason : FutureIncompatibilityReason :: FutureReleaseError , explain_reason : true , report_in_deps : false , } } }
}