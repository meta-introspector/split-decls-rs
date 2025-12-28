use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_1_Shl_1 () { type A = UInt < UTerm , B1 > ; type B = UInt < UTerm , B1 > ; type U2 = UInt < UInt < UTerm , B1 > , B0 > ; # [allow (non_camel_case_types)] type U1ShlU1 = < < A as Shl < B > > :: Output as Same < U2 > > :: Output ; assert_eq ! (< U1ShlU1 as Unsigned >:: to_u64 () , < U2 as Unsigned >:: to_u64 ()) ; }
}