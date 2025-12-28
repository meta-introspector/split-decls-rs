use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_5_Shl_4");
# [test] # [allow (non_snake_case)] fn test_5_Shl_4 () { type A = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; type B = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; type U80 = UInt < UInt < UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > , B0 > , B0 > , B0 > , B0 > ; # [allow (non_camel_case_types)] type U5ShlU4 = < < A as Shl < B > > :: Output as Same < U80 > > :: Output ; assert_eq ! (< U5ShlU4 as Unsigned >:: to_u64 () , < U80 as Unsigned >:: to_u64 ()) ; }
}