use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_3_Shl_5 () { type A = UInt < UInt < UTerm , B1 > , B1 > ; type B = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; type U96 = UInt < UInt < UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B0 > , B0 > , B0 > , B0 > , B0 > ; # [allow (non_camel_case_types)] type U3ShlU5 = < < A as Shl < B > > :: Output as Same < U96 > > :: Output ; assert_eq ! (< U3ShlU5 as Unsigned >:: to_u64 () , < U96 as Unsigned >:: to_u64 ()) ; }