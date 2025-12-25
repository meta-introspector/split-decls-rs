use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub enum TestEnum {
    Variant1,
    Variant2(i32),
}
