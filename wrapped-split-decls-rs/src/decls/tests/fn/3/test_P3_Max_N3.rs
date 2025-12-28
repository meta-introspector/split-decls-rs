use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P3_Max_N3 () { type A = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type P3 = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; # [allow (non_camel_case_types)] type P3MaxN3 = < < A as Max < B > > :: Output as Same < P3 > > :: Output ; assert_eq ! (< P3MaxN3 as Integer >:: to_i64 () , < P3 as Integer >:: to_i64 ()) ; }