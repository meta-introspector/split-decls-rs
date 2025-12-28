use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test__0_Gcd_P1 () { type A = Z0 ; type B = PInt < UInt < UTerm , B1 > > ; type P1 = PInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type _0GcdP1 = < < A as Gcd < B > > :: Output as Same < P1 > > :: Output ; assert_eq ! (< _0GcdP1 as Integer >:: to_i64 () , < P1 as Integer >:: to_i64 ()) ; }
}