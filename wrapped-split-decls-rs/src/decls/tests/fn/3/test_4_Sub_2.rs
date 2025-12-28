use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_4_Sub_2 () { type A = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; type B = UInt < UInt < UTerm , B1 > , B0 > ; type U2 = UInt < UInt < UTerm , B1 > , B0 > ; # [allow (non_camel_case_types)] type U4SubU2 = < < A as Sub < B > > :: Output as Same < U2 > > :: Output ; assert_eq ! (< U4SubU2 as Unsigned >:: to_u64 () , < U2 as Unsigned >:: to_u64 ()) ; }