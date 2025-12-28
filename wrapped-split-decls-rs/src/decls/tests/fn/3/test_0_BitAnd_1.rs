use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_0_BitAnd_1");
# [test] # [allow (non_snake_case)] fn test_0_BitAnd_1 () { type A = UTerm ; type B = UInt < UTerm , B1 > ; type U0 = UTerm ; # [allow (non_camel_case_types)] type U0BitAndU1 = < < A as BitAnd < B > > :: Output as Same < U0 > > :: Output ; assert_eq ! (< U0BitAndU1 as Unsigned >:: to_u64 () , < U0 as Unsigned >:: to_u64 ()) ; }
}