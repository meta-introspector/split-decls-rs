use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_P1_Neg");
# [test] # [allow (non_snake_case)] fn test_P1_Neg () { type A = PInt < UInt < UTerm , B1 > > ; type N1 = NInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type NegP1 = < < A as Neg > :: Output as Same < N1 > > :: Output ; assert_eq ! (< NegP1 as Integer >:: to_i64 () , < N1 as Integer >:: to_i64 ()) ; }
}