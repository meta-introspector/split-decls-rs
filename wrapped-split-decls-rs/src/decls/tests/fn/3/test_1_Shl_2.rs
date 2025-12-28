use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_1_Shl_2 () { type A = UInt < UTerm , B1 > ; type B = UInt < UInt < UTerm , B1 > , B0 > ; type U4 = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; # [allow (non_camel_case_types)] type U1ShlU2 = < < A as Shl < B > > :: Output as Same < U4 > > :: Output ; assert_eq ! (< U1ShlU2 as Unsigned >:: to_u64 () , < U4 as Unsigned >:: to_u64 ()) ; }