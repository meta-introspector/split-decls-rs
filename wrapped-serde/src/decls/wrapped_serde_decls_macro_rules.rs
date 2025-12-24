use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[macro_export]
#[doc(hidden)]
macro_rules! __require_serde_not_serde_core {
    () => {};
}
