use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test__0_Div_P1");
# [test] # [allow (non_snake_case)] fn test__0_Div_P1 () { type A = Z0 ; type B = PInt < UInt < UTerm , B1 > > ; type _0 = Z0 ; # [allow (non_camel_case_types)] type _0DivP1 = < < A as Div < B > > :: Output as Same < _0 > > :: Output ; assert_eq ! (< _0DivP1 as Integer >:: to_i64 () , < _0 as Integer >:: to_i64 ()) ; }
}