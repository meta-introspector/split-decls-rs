use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_2_Div_4 () { type A = UInt < UInt < UTerm , B1 > , B0 > ; type B = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; type U0 = UTerm ; # [allow (non_camel_case_types)] type U2DivU4 = < < A as Div < B > > :: Output as Same < U0 > > :: Output ; assert_eq ! (< U2DivU4 as Unsigned >:: to_u64 () , < U0 as Unsigned >:: to_u64 ()) ; }