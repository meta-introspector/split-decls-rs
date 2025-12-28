use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Grid { fn next (self) -> Option < Grid > { use crate :: Grid :: * ; match self { Major => Some (Minor) , Minor => None , } } }