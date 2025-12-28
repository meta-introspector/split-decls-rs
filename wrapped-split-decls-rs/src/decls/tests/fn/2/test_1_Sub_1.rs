use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_1_Sub_1 () { type A = UInt < UTerm , B1 > ; type B = UInt < UTerm , B1 > ; type U0 = UTerm ; # [allow (non_camel_case_types)] type U1SubU1 = < < A as Sub < B > > :: Output as Same < U0 > > :: Output ; assert_eq ! (< U1SubU1 as Unsigned >:: to_u64 () , < U0 as Unsigned >:: to_u64 ()) ; }