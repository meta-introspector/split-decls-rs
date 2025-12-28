use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl DwimIntent { fn hash (& self) -> String { format ! ("{:x}" , self . keywords . join ("") . len ()) } }