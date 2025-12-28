use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N1_Sub_N2 () { type A = NInt < UInt < UTerm , B1 > > ; type B = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type P1 = PInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type N1SubN2 = < < A as Sub < B > > :: Output as Same < P1 > > :: Output ; assert_eq ! (< N1SubN2 as Integer >:: to_i64 () , < P1 as Integer >:: to_i64 ()) ; }