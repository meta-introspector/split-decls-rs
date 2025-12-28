use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_1_Add_3 () { type A = UInt < UTerm , B1 > ; type B = UInt < UInt < UTerm , B1 > , B1 > ; type U4 = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; # [allow (non_camel_case_types)] type U1AddU3 = < < A as Add < B > > :: Output as Same < U4 > > :: Output ; assert_eq ! (< U1AddU3 as Unsigned >:: to_u64 () , < U4 as Unsigned >:: to_u64 ()) ; }