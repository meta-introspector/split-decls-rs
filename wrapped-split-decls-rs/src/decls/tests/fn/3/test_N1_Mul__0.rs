use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_N1_Mul__0");
# [test] # [allow (non_snake_case)] fn test_N1_Mul__0 () { type A = NInt < UInt < UTerm , B1 > > ; type B = Z0 ; type _0 = Z0 ; # [allow (non_camel_case_types)] type N1Mul_0 = < < A as Mul < B > > :: Output as Same < _0 > > :: Output ; assert_eq ! (< N1Mul_0 as Integer >:: to_i64 () , < _0 as Integer >:: to_i64 ()) ; }
}