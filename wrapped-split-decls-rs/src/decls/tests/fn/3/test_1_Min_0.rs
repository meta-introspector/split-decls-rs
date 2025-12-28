use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_1_Min_0");
# [test] # [allow (non_snake_case)] fn test_1_Min_0 () { type A = UInt < UTerm , B1 > ; type B = UTerm ; type U0 = UTerm ; # [allow (non_camel_case_types)] type U1MinU0 = < < A as Min < B > > :: Output as Same < U0 > > :: Output ; assert_eq ! (< U1MinU0 as Unsigned >:: to_u64 () , < U0 as Unsigned >:: to_u64 ()) ; }
}