use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl HasDataLayout for TargetDataLayout { # [inline] fn data_layout (& self) -> & TargetDataLayout { self } }