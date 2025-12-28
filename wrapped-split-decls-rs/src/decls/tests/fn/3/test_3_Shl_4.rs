use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_3_Shl_4 () { type A = UInt < UInt < UTerm , B1 > , B1 > ; type B = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; type U48 = UInt < UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B0 > , B0 > , B0 > , B0 > ; # [allow (non_camel_case_types)] type U3ShlU4 = < < A as Shl < B > > :: Output as Same < U48 > > :: Output ; assert_eq ! (< U3ShlU4 as Unsigned >:: to_u64 () , < U48 as Unsigned >:: to_u64 ()) ; }