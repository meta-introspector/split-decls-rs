use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Axis { fn next (self) -> Option < Axis > { use crate :: Axis :: * ; match self { BottomX => Some (LeftY) , LeftY => Some (RightY) , RightY => Some (TopX) , TopX => None , } } }