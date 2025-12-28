use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_N5_Mul_N5");
# [test] # [allow (non_snake_case)] fn test_N5_Mul_N5 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type P25 = PInt < UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B0 > , B0 > , B1 > > ; # [allow (non_camel_case_types)] type N5MulN5 = < < A as Mul < B > > :: Output as Same < P25 > > :: Output ; assert_eq ! (< N5MulN5 as Integer >:: to_i64 () , < P25 as Integer >:: to_i64 ()) ; }
}