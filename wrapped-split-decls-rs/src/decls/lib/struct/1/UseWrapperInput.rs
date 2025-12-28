use serde::{Deserialize, Serialize};
use std::collections::HashMap;

struct UseWrapperInput { original_path : syn :: Path , wrapper_path : syn :: Path , }