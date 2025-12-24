use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Retrieve all items in the local crate that have a MIR associated with them.
pub fn all_local_items() -> CrateItems {
    with(|cx| cx.all_local_items())
}
