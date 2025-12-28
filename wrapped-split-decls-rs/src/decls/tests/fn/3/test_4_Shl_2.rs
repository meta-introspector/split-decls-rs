use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_4_Shl_2 () { type A = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; type B = UInt < UInt < UTerm , B1 > , B0 > ; type U16 = UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > , B0 > , B0 > ; # [allow (non_camel_case_types)] type U4ShlU2 = < < A as Shl < B > > :: Output as Same < U16 > > :: Output ; assert_eq ! (< U4ShlU2 as Unsigned >:: to_u64 () , < U16 as Unsigned >:: to_u64 ()) ; }