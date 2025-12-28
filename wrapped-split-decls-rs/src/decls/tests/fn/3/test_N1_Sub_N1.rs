use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N1_Sub_N1 () { type A = NInt < UInt < UTerm , B1 > > ; type B = NInt < UInt < UTerm , B1 > > ; type _0 = Z0 ; # [allow (non_camel_case_types)] type N1SubN1 = < < A as Sub < B > > :: Output as Same < _0 > > :: Output ; assert_eq ! (< N1SubN1 as Integer >:: to_i64 () , < _0 as Integer >:: to_i64 ()) ; }