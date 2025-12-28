use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_5_Shl_1 () { type A = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; type B = UInt < UTerm , B1 > ; type U10 = UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > , B0 > ; # [allow (non_camel_case_types)] type U5ShlU1 = < < A as Shl < B > > :: Output as Same < U10 > > :: Output ; assert_eq ! (< U5ShlU1 as Unsigned >:: to_u64 () , < U10 as Unsigned >:: to_u64 ()) ; }