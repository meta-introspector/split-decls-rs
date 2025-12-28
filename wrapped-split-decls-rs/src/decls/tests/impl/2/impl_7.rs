use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Maxes { fn annotation (& self , scc : usize) -> MaxReached { self . 0 [scc] } fn new (mapping : fn (usize) -> usize) -> Self { Self (IndexVec :: new () , mapping) } }