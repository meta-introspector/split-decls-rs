use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_3_Shl_2 () { type A = UInt < UInt < UTerm , B1 > , B1 > ; type B = UInt < UInt < UTerm , B1 > , B0 > ; type U12 = UInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B0 > , B0 > ; # [allow (non_camel_case_types)] type U3ShlU2 = < < A as Shl < B > > :: Output as Same < U12 > > :: Output ; assert_eq ! (< U3ShlU2 as Unsigned >:: to_u64 () , < U12 as Unsigned >:: to_u64 ()) ; }
}