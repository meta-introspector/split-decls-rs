use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N5_Max_P5 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type P5 = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; # [allow (non_camel_case_types)] type N5MaxP5 = < < A as Max < B > > :: Output as Same < P5 > > :: Output ; assert_eq ! (< N5MaxP5 as Integer >:: to_i64 () , < P5 as Integer >:: to_i64 ()) ; }