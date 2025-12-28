use serde::{Deserialize, Serialize};
use std::collections::HashMap;

unsafe impl < T : DynSend > Send for FromDyn < T > { }