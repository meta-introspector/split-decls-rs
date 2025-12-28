use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl MetaPatternCounter { pub fn new () -> Self { Self { counts : std :: collections :: HashMap :: new () } } pub fn increment (& mut self , pattern : & str) { * self . counts . entry (pattern . to_string ()) . or_insert (0) += 1 ; } pub fn report (& self) { println ! ("🎯 META PATTERN ANALYSIS:") ; for (pattern , count) in & self . counts { println ! ("  {}: {}" , pattern , count) ; } } }