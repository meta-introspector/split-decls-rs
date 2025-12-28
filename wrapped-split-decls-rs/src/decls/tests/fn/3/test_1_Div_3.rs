use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_1_Div_3 () { type A = UInt < UTerm , B1 > ; type B = UInt < UInt < UTerm , B1 > , B1 > ; type U0 = UTerm ; # [allow (non_camel_case_types)] type U1DivU3 = < < A as Div < B > > :: Output as Same < U0 > > :: Output ; assert_eq ! (< U1DivU3 as Unsigned >:: to_u64 () , < U0 as Unsigned >:: to_u64 ()) ; }