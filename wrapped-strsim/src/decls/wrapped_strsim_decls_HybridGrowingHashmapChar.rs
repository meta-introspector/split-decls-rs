use serde::{Deserialize, Serialize};
use std::collections::HashMap;
struct HybridGrowingHashmapChar<ValueType> {
    map: GrowingHashmapChar<ValueType>,
    extended_ascii: [ValueType; 256],
}
