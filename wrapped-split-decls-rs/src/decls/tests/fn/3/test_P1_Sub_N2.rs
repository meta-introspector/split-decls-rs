use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P1_Sub_N2 () { type A = PInt < UInt < UTerm , B1 > > ; type B = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type P3 = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; # [allow (non_camel_case_types)] type P1SubN2 = < < A as Sub < B > > :: Output as Same < P3 > > :: Output ; assert_eq ! (< P1SubN2 as Integer >:: to_i64 () , < P3 as Integer >:: to_i64 ()) ; }