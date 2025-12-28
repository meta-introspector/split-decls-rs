use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_5_Shl_5 () { type A = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; type B = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; type U160 = UInt < UInt < UInt < UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > , B0 > , B0 > , B0 > , B0 > , B0 > ; # [allow (non_camel_case_types)] type U5ShlU5 = < < A as Shl < B > > :: Output as Same < U160 > > :: Output ; assert_eq ! (< U5ShlU5 as Unsigned >:: to_u64 () , < U160 as Unsigned >:: to_u64 ()) ; }
}