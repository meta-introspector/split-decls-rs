use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_4_Shl_3 () { type A = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; type B = UInt < UInt < UTerm , B1 > , B1 > ; type U32 = UInt < UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > , B0 > , B0 > , B0 > ; # [allow (non_camel_case_types)] type U4ShlU3 = < < A as Shl < B > > :: Output as Same < U32 > > :: Output ; assert_eq ! (< U4ShlU3 as Unsigned >:: to_u64 () , < U32 as Unsigned >:: to_u64 ()) ; }