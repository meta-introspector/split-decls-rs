use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_1_Mul_4 () { type A = UInt < UTerm , B1 > ; type B = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; type U4 = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; # [allow (non_camel_case_types)] type U1MulU4 = < < A as Mul < B > > :: Output as Same < U4 > > :: Output ; assert_eq ! (< U1MulU4 as Unsigned >:: to_u64 () , < U4 as Unsigned >:: to_u64 ()) ; }
}