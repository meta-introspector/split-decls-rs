use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N4_Sub_N4 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type _0 = Z0 ; # [allow (non_camel_case_types)] type N4SubN4 = < < A as Sub < B > > :: Output as Same < _0 > > :: Output ; assert_eq ! (< N4SubN4 as Integer >:: to_i64 () , < _0 as Integer >:: to_i64 ()) ; }