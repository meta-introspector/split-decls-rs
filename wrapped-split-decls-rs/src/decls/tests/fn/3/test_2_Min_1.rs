use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_2_Min_1 () { type A = UInt < UInt < UTerm , B1 > , B0 > ; type B = UInt < UTerm , B1 > ; type U1 = UInt < UTerm , B1 > ; # [allow (non_camel_case_types)] type U2MinU1 = < < A as Min < B > > :: Output as Same < U1 > > :: Output ; assert_eq ! (< U2MinU1 as Unsigned >:: to_u64 () , < U1 as Unsigned >:: to_u64 ()) ; }