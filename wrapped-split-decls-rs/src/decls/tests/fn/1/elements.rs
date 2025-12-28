use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn elements (s : SortedMap < u32 , u32 >) -> Vec < (u32 , u32) > { s . into_iter () . collect :: < Vec < (u32 , u32) > > () }