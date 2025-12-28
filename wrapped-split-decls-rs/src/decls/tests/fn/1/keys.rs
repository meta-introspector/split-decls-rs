use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn keys (s : SortedMap < u32 , u32 >) -> Vec < u32 > { s . into_iter () . map (| (k , _) | k) . collect :: < Vec < u32 > > () }