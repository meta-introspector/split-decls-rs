use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test__0_Gcd__0");
# [test] # [allow (non_snake_case)] fn test__0_Gcd__0 () { type A = Z0 ; type B = Z0 ; type _0 = Z0 ; # [allow (non_camel_case_types)] type _0Gcd_0 = < < A as Gcd < B > > :: Output as Same < _0 > > :: Output ; assert_eq ! (< _0Gcd_0 as Integer >:: to_i64 () , < _0 as Integer >:: to_i64 ()) ; }
}