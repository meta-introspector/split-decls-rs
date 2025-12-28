use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_4_Shl_4 () { type A = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; type B = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; type U64 = UInt < UInt < UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > , B0 > , B0 > , B0 > , B0 > ; # [allow (non_camel_case_types)] type U4ShlU4 = < < A as Shl < B > > :: Output as Same < U64 > > :: Output ; assert_eq ! (< U4ShlU4 as Unsigned >:: to_u64 () , < U64 as Unsigned >:: to_u64 ()) ; }
}