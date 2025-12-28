use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_0_Pow_0");
# [test] # [allow (non_snake_case)] fn test_0_Pow_0 () { type A = UTerm ; type B = UTerm ; type U1 = UInt < UTerm , B1 > ; # [allow (non_camel_case_types)] type U0PowU0 = < < A as Pow < B > > :: Output as Same < U1 > > :: Output ; assert_eq ! (< U0PowU0 as Unsigned >:: to_u64 () , < U1 as Unsigned >:: to_u64 ()) ; }
}