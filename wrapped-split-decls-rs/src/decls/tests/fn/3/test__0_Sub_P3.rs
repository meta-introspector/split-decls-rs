use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test__0_Sub_P3 () { type A = Z0 ; type B = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type N3 = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; # [allow (non_camel_case_types)] type _0SubP3 = < < A as Sub < B > > :: Output as Same < N3 > > :: Output ; assert_eq ! (< _0SubP3 as Integer >:: to_i64 () , < N3 as Integer >:: to_i64 ()) ; }