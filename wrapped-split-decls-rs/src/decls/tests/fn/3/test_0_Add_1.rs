use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_0_Add_1");
# [test] # [allow (non_snake_case)] fn test_0_Add_1 () { type A = UTerm ; type B = UInt < UTerm , B1 > ; type U1 = UInt < UTerm , B1 > ; # [allow (non_camel_case_types)] type U0AddU1 = < < A as Add < B > > :: Output as Same < U1 > > :: Output ; assert_eq ! (< U0AddU1 as Unsigned >:: to_u64 () , < U1 as Unsigned >:: to_u64 ()) ; }
}