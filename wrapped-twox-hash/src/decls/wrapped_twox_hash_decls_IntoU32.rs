use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[allow(dead_code, reason = "Too lazy to cfg-gate these")]
trait IntoU32 {
    fn into_u32(self) -> u32;
}
