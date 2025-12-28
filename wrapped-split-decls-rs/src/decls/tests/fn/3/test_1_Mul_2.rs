use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_1_Mul_2 () { type A = UInt < UTerm , B1 > ; type B = UInt < UInt < UTerm , B1 > , B0 > ; type U2 = UInt < UInt < UTerm , B1 > , B0 > ; # [allow (non_camel_case_types)] type U1MulU2 = < < A as Mul < B > > :: Output as Same < U2 > > :: Output ; assert_eq ! (< U1MulU2 as Unsigned >:: to_u64 () , < U2 as Unsigned >:: to_u64 ()) ; }