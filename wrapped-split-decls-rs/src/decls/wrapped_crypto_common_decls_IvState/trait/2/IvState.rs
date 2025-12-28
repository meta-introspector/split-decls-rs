use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Trait for loading current IV state."] pub trait IvState : IvSizeUser { # [doc = " Returns current IV state."] fn iv_state (& self) -> Iv < Self > ; }