use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N1_Sub_N4 () { type A = NInt < UInt < UTerm , B1 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type P3 = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; # [allow (non_camel_case_types)] type N1SubN4 = < < A as Sub < B > > :: Output as Same < P3 > > :: Output ; assert_eq ! (< N1SubN4 as Integer >:: to_i64 () , < P3 as Integer >:: to_i64 ()) ; }