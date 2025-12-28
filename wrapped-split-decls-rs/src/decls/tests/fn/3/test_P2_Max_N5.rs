use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P2_Max_N5 () { type A = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type P2 = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; # [allow (non_camel_case_types)] type P2MaxN5 = < < A as Max < B > > :: Output as Same < P2 > > :: Output ; assert_eq ! (< P2MaxN5 as Integer >:: to_i64 () , < P2 as Integer >:: to_i64 ()) ; }