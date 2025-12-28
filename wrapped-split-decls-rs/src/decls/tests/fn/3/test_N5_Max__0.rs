use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N5_Max__0 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type B = Z0 ; type _0 = Z0 ; # [allow (non_camel_case_types)] type N5Max_0 = < < A as Max < B > > :: Output as Same < _0 > > :: Output ; assert_eq ! (< N5Max_0 as Integer >:: to_i64 () , < _0 as Integer >:: to_i64 ()) ; }
}