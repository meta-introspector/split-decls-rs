use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_P4_Mul_P5");
# [test] # [allow (non_snake_case)] fn test_P4_Mul_P5 () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type P20 = PInt < UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type P4MulP5 = < < A as Mul < B > > :: Output as Same < P20 > > :: Output ; assert_eq ! (< P4MulP5 as Integer >:: to_i64 () , < P20 as Integer >:: to_i64 ()) ; }
}