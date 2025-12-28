use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_1_Shl_3 () { type A = UInt < UTerm , B1 > ; type B = UInt < UInt < UTerm , B1 > , B1 > ; type U8 = UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > , B0 > ; # [allow (non_camel_case_types)] type U1ShlU3 = < < A as Shl < B > > :: Output as Same < U8 > > :: Output ; assert_eq ! (< U1ShlU3 as Unsigned >:: to_u64 () , < U8 as Unsigned >:: to_u64 ()) ; }