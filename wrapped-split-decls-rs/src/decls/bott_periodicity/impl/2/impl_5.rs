use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Level8DPoint { pub fn new (coords : [f64 ; 8]) -> Self { Self { coordinates : coords , level : 0 , generation : 0 , cached_result : None , } } }