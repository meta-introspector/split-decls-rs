use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_4_Mul_3 () { type A = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; type B = UInt < UInt < UTerm , B1 > , B1 > ; type U12 = UInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B0 > , B0 > ; # [allow (non_camel_case_types)] type U4MulU3 = < < A as Mul < B > > :: Output as Same < U12 > > :: Output ; assert_eq ! (< U4MulU3 as Unsigned >:: to_u64 () , < U12 as Unsigned >:: to_u64 ()) ; }
}