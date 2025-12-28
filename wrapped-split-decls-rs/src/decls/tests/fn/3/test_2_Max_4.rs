use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_2_Max_4 () { type A = UInt < UInt < UTerm , B1 > , B0 > ; type B = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; type U4 = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; # [allow (non_camel_case_types)] type U2MaxU4 = < < A as Max < B > > :: Output as Same < U4 > > :: Output ; assert_eq ! (< U2MaxU4 as Unsigned >:: to_u64 () , < U4 as Unsigned >:: to_u64 ()) ; }