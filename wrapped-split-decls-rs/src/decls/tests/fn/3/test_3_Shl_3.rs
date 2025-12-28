use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_3_Shl_3 () { type A = UInt < UInt < UTerm , B1 > , B1 > ; type B = UInt < UInt < UTerm , B1 > , B1 > ; type U24 = UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B0 > , B0 > , B0 > ; # [allow (non_camel_case_types)] type U3ShlU3 = < < A as Shl < B > > :: Output as Same < U24 > > :: Output ; assert_eq ! (< U3ShlU3 as Unsigned >:: to_u64 () , < U24 as Unsigned >:: to_u64 ()) ; }