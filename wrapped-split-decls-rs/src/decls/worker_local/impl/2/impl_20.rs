use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T : Default > Default for WorkerLocal < T > { fn default () -> Self { WorkerLocal :: new (| _ | T :: default ()) } }
}