use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_4_Max_2 () { type A = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; type B = UInt < UInt < UTerm , B1 > , B0 > ; type U4 = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; # [allow (non_camel_case_types)] type U4MaxU2 = < < A as Max < B > > :: Output as Same < U4 > > :: Output ; assert_eq ! (< U4MaxU2 as Unsigned >:: to_u64 () , < U4 as Unsigned >:: to_u64 ()) ; }