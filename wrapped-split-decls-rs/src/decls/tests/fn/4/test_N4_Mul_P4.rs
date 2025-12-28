use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_N4_Mul_P4");
# [test] # [allow (non_snake_case)] fn test_N4_Mul_P4 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type N16 = NInt < UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type N4MulP4 = < < A as Mul < B > > :: Output as Same < N16 > > :: Output ; assert_eq ! (< N4MulP4 as Integer >:: to_i64 () , < N16 as Integer >:: to_i64 ()) ; }
}