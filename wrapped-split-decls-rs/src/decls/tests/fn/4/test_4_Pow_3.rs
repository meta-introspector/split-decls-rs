use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_4_Pow_3");
# [test] # [allow (non_snake_case)] fn test_4_Pow_3 () { type A = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; type B = UInt < UInt < UTerm , B1 > , B1 > ; type U64 = UInt < UInt < UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > , B0 > , B0 > , B0 > , B0 > ; # [allow (non_camel_case_types)] type U4PowU3 = < < A as Pow < B > > :: Output as Same < U64 > > :: Output ; assert_eq ! (< U4PowU3 as Unsigned >:: to_u64 () , < U64 as Unsigned >:: to_u64 ()) ; }
}