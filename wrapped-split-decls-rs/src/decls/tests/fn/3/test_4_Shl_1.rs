use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_4_Shl_1 () { type A = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; type B = UInt < UTerm , B1 > ; type U8 = UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > , B0 > ; # [allow (non_camel_case_types)] type U4ShlU1 = < < A as Shl < B > > :: Output as Same < U8 > > :: Output ; assert_eq ! (< U4ShlU1 as Unsigned >:: to_u64 () , < U8 as Unsigned >:: to_u64 ()) ; }
}