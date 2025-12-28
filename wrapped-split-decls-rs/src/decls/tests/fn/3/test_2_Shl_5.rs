use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_2_Shl_5 () { type A = UInt < UInt < UTerm , B1 > , B0 > ; type B = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; type U64 = UInt < UInt < UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > , B0 > , B0 > , B0 > , B0 > ; # [allow (non_camel_case_types)] type U2ShlU5 = < < A as Shl < B > > :: Output as Same < U64 > > :: Output ; assert_eq ! (< U2ShlU5 as Unsigned >:: to_u64 () , < U64 as Unsigned >:: to_u64 ()) ; }
}