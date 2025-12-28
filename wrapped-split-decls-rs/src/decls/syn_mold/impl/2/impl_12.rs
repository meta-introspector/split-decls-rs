use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl SynUsageVisitor { pub fn new () -> Self { Self { patterns : HashMap :: new () , operations : Vec :: new () , parse_calls : 0 , visit_calls : 0 , transform_calls : 0 , generation_calls : 0 , } } }