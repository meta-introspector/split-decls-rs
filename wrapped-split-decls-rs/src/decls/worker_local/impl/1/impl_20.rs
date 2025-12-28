use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T : Default > Default for WorkerLocal < T > { fn default () -> Self { WorkerLocal :: new (| _ | T :: default ()) } }