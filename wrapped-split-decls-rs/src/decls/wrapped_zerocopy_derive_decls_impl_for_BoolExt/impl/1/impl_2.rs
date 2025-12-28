use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl BoolExt for bool { fn then_some < T > (self , t : T) -> Option < T > { if self { Some (t) } else { None } } }