use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Annotation for MinMaxIn { fn merge_scc (self , other : Self) -> Self { Self { min : std :: cmp :: min (self . min , other . min) , max : std :: cmp :: max (self . max , other . max) } } fn merge_reached (self , _other : Self) -> Self { self } }