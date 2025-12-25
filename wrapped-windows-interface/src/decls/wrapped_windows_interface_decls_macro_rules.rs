use serde::{Deserialize, Serialize};
use std::collections::HashMap;
macro_rules! expected_token {
    ($sig:tt .$item:tt (), $msg:expr) => {
        if let None = $sig.$item() {
            bail!($sig, "expected {}", $msg);
        }
    };
}
