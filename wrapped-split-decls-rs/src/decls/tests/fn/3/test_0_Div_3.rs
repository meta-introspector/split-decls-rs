use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_0_Div_3");
# [test] # [allow (non_snake_case)] fn test_0_Div_3 () { type A = UTerm ; type B = UInt < UInt < UTerm , B1 > , B1 > ; type U0 = UTerm ; # [allow (non_camel_case_types)] type U0DivU3 = < < A as Div < B > > :: Output as Same < U0 > > :: Output ; assert_eq ! (< U0DivU3 as Unsigned >:: to_u64 () , < U0 as Unsigned >:: to_u64 ()) ; }
}