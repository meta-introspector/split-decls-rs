use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N2_Max__0 () { type A = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = Z0 ; type _0 = Z0 ; # [allow (non_camel_case_types)] type N2Max_0 = < < A as Max < B > > :: Output as Same < _0 > > :: Output ; assert_eq ! (< N2Max_0 as Integer >:: to_i64 () , < _0 as Integer >:: to_i64 ()) ; }
}