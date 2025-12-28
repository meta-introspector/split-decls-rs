use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " A dummy implementation of `BottPeriodicityTrait` for testing."] # [derive (Debug , Default , Clone , PartialEq , Serialize , Deserialize)] pub struct DummyBottPeriodicity { data : BottPeriodicityData , }
}