use serde::{Deserialize, Serialize};
use std::collections::HashMap;
rustc_index::newtype_index! {
    #[cfg_attr(feature = "nightly", derive(HashStable_NoContext))] #[encodable]
    #[orderable] #[debug_format = "{}"] #[gate_rustc_only] pub struct BoundVar {}
}
