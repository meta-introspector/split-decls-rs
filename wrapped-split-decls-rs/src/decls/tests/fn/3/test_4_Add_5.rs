use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_4_Add_5 () { type A = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; type B = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; type U9 = UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > , B1 > ; # [allow (non_camel_case_types)] type U4AddU5 = < < A as Add < B > > :: Output as Same < U9 > > :: Output ; assert_eq ! (< U4AddU5 as Unsigned >:: to_u64 () , < U9 as Unsigned >:: to_u64 ()) ; }