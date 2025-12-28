use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_5_Mul_4 () { type A = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; type B = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; type U20 = UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > , B0 > , B0 > ; # [allow (non_camel_case_types)] type U5MulU4 = < < A as Mul < B > > :: Output as Same < U20 > > :: Output ; assert_eq ! (< U5MulU4 as Unsigned >:: to_u64 () , < U20 as Unsigned >:: to_u64 ()) ; }