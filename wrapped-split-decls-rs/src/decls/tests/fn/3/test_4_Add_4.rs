use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_4_Add_4 () { type A = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; type B = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; type U8 = UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > , B0 > ; # [allow (non_camel_case_types)] type U4AddU4 = < < A as Add < B > > :: Output as Same < U8 > > :: Output ; assert_eq ! (< U4AddU4 as Unsigned >:: to_u64 () , < U8 as Unsigned >:: to_u64 ()) ; }