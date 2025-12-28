use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test__0_Sub_P1 () { type A = Z0 ; type B = PInt < UInt < UTerm , B1 > > ; type N1 = NInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type _0SubP1 = < < A as Sub < B > > :: Output as Same < N1 > > :: Output ; assert_eq ! (< _0SubP1 as Integer >:: to_i64 () , < N1 as Integer >:: to_i64 ()) ; }