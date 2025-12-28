use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N4_Max_P3 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type P3 = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; # [allow (non_camel_case_types)] type N4MaxP3 = < < A as Max < B > > :: Output as Same < P3 > > :: Output ; assert_eq ! (< N4MaxP3 as Integer >:: to_i64 () , < P3 as Integer >:: to_i64 ()) ; }