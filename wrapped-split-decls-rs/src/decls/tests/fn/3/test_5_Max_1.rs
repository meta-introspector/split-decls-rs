use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_5_Max_1 () { type A = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; type B = UInt < UTerm , B1 > ; type U5 = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; # [allow (non_camel_case_types)] type U5MaxU1 = < < A as Max < B > > :: Output as Same < U5 > > :: Output ; assert_eq ! (< U5MaxU1 as Unsigned >:: to_u64 () , < U5 as Unsigned >:: to_u64 ()) ; }