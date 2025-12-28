use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P1_Abs () { type A = PInt < UInt < UTerm , B1 > > ; type P1 = PInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type AbsP1 = < < A as Abs > :: Output as Same < P1 > > :: Output ; assert_eq ! (< AbsP1 as Integer >:: to_i64 () , < P1 as Integer >:: to_i64 ()) ; }