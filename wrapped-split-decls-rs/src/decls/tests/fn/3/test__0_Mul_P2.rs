use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test__0_Mul_P2 () { type A = Z0 ; type B = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type _0 = Z0 ; # [allow (non_camel_case_types)] type _0MulP2 = < < A as Mul < B > > :: Output as Same < _0 > > :: Output ; assert_eq ! (< _0MulP2 as Integer >:: to_i64 () , < _0 as Integer >:: to_i64 ()) ; }
}