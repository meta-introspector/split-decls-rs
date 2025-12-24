use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl IntoU128 for u64 {
    fn into_u128(self) -> u128 {
        u128::from(self)
    }
}
