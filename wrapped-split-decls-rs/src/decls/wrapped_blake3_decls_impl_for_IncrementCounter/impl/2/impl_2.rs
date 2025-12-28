use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl IncrementCounter { # [inline] fn yes (& self) -> bool { match self { IncrementCounter :: Yes => true , IncrementCounter :: No => false , } } }