use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P1_Max_N3 () { type A = PInt < UInt < UTerm , B1 > > ; type B = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type P1 = PInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type P1MaxN3 = < < A as Max < B > > :: Output as Same < P1 > > :: Output ; assert_eq ! (< P1MaxN3 as Integer >:: to_i64 () , < P1 as Integer >:: to_i64 ()) ; }