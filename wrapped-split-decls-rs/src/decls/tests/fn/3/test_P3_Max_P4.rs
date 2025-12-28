use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P3_Max_P4 () { type A = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type P4 = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type P3MaxP4 = < < A as Max < B > > :: Output as Same < P4 > > :: Output ; assert_eq ! (< P3MaxP4 as Integer >:: to_i64 () , < P4 as Integer >:: to_i64 ()) ; }