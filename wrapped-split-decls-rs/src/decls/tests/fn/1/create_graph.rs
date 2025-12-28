use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn create_graph () -> VecGraph < usize > { VecGraph :: new (7 , vec ! [(0 , 1) , (1 , 2) , (1 , 3) , (3 , 4) , (5 , 1)]) }