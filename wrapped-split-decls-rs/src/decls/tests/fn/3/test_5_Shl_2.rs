use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_5_Shl_2 () { type A = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; type B = UInt < UInt < UTerm , B1 > , B0 > ; type U20 = UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > , B0 > , B0 > ; # [allow (non_camel_case_types)] type U5ShlU2 = < < A as Shl < B > > :: Output as Same < U20 > > :: Output ; assert_eq ! (< U5ShlU2 as Unsigned >:: to_u64 () , < U20 as Unsigned >:: to_u64 ()) ; }