use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_N5_Mul_P5");
# [test] # [allow (non_snake_case)] fn test_N5_Mul_P5 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type N25 = NInt < UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B0 > , B0 > , B1 > > ; # [allow (non_camel_case_types)] type N5MulP5 = < < A as Mul < B > > :: Output as Same < N25 > > :: Output ; assert_eq ! (< N5MulP5 as Integer >:: to_i64 () , < N25 as Integer >:: to_i64 ()) ; }
}