use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Cache for Bott periodicity computations"] # [derive (Debug , Clone)] pub struct BottPeriodicityCache { cache : std :: collections :: HashMap < usize , AbstractionBundle > , pub current_generation : usize , pub levels : Vec < AbstractionBundle > , pub fiber_bundles : Vec < String > , }
}