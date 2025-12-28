use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_4_Shl_5 () { type A = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; type B = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; type U128 = UInt < UInt < UInt < UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > , B0 > , B0 > , B0 > , B0 > , B0 > ; # [allow (non_camel_case_types)] type U4ShlU5 = < < A as Shl < B > > :: Output as Same < U128 > > :: Output ; assert_eq ! (< U4ShlU5 as Unsigned >:: to_u64 () , < U128 as Unsigned >:: to_u64 ()) ; }
}