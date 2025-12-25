use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn as_bytes(value: &HSTRING) -> &[u8] {
    unsafe {
        core::slice::from_raw_parts(value.as_ptr() as *const _, (value.len() + 1) * 2)
    }
}
