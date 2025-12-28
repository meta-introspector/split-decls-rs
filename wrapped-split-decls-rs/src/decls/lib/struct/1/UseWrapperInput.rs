use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct UseWrapperInput { original_path : syn :: Path , wrapper_path : syn :: Path , }
}