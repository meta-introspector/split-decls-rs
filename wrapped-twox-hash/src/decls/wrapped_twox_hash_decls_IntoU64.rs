use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[allow(dead_code, reason = "Too lazy to cfg-gate these")]
trait IntoU64 {
    fn into_u64(self) -> u64;
}
