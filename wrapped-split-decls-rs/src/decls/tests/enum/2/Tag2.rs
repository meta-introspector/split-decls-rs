use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A tag type used in [`TaggedRef`] tests."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] enum Tag2 { B00 = 0b00 , B01 = 0b01 , B10 = 0b10 , B11 = 0b11 , }