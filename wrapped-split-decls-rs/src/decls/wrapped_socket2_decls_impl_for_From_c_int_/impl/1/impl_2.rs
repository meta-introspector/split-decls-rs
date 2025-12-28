use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl From < c_int > for Protocol { fn from (p : c_int) -> Protocol { Protocol (p) } }