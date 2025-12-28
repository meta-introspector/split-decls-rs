use serde::{Deserialize, Serialize};
use std::collections::HashMap;

unsafe impl < T : Send , N : ArrayLength > Send for GenericArray < T , N > { }