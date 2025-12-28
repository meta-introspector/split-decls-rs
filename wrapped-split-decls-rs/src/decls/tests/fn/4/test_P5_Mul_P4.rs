use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_P5_Mul_P4");
# [test] # [allow (non_snake_case)] fn test_P5_Mul_P4 () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type P20 = PInt < UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type P5MulP4 = < < A as Mul < B > > :: Output as Same < P20 > > :: Output ; assert_eq ! (< P5MulP4 as Integer >:: to_i64 () , < P20 as Integer >:: to_i64 ()) ; }
}