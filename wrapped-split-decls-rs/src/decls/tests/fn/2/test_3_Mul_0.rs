use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_3_Mul_0 () { type A = UInt < UInt < UTerm , B1 > , B1 > ; type B = UTerm ; type U0 = UTerm ; # [allow (non_camel_case_types)] type U3MulU0 = < < A as Mul < B > > :: Output as Same < U0 > > :: Output ; assert_eq ! (< U3MulU0 as Unsigned >:: to_u64 () , < U0 as Unsigned >:: to_u64 ()) ; }