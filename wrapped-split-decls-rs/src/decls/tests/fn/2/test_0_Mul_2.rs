use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_0_Mul_2 () { type A = UTerm ; type B = UInt < UInt < UTerm , B1 > , B0 > ; type U0 = UTerm ; # [allow (non_camel_case_types)] type U0MulU2 = < < A as Mul < B > > :: Output as Same < U0 > > :: Output ; assert_eq ! (< U0MulU2 as Unsigned >:: to_u64 () , < U0 as Unsigned >:: to_u64 ()) ; }