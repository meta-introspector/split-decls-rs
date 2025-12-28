use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test__0_Sub_N1 () { type A = Z0 ; type B = NInt < UInt < UTerm , B1 > > ; type P1 = PInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type _0SubN1 = < < A as Sub < B > > :: Output as Same < P1 > > :: Output ; assert_eq ! (< _0SubN1 as Integer >:: to_i64 () , < P1 as Integer >:: to_i64 ()) ; }