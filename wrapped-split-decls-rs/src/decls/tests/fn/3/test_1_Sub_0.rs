use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_1_Sub_0");
# [test] # [allow (non_snake_case)] fn test_1_Sub_0 () { type A = UInt < UTerm , B1 > ; type B = UTerm ; type U1 = UInt < UTerm , B1 > ; # [allow (non_camel_case_types)] type U1SubU0 = < < A as Sub < B > > :: Output as Same < U1 > > :: Output ; assert_eq ! (< U1SubU0 as Unsigned >:: to_u64 () , < U1 as Unsigned >:: to_u64 ()) ; }
}