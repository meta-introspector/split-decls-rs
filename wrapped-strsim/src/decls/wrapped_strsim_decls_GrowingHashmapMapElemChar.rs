use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Default, Clone)]
struct GrowingHashmapMapElemChar<ValueType> {
    key: u32,
    value: ValueType,
}
