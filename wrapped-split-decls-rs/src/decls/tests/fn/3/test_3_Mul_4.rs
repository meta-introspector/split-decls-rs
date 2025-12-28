use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_3_Mul_4 () { type A = UInt < UInt < UTerm , B1 > , B1 > ; type B = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; type U12 = UInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B0 > , B0 > ; # [allow (non_camel_case_types)] type U3MulU4 = < < A as Mul < B > > :: Output as Same < U12 > > :: Output ; assert_eq ! (< U3MulU4 as Unsigned >:: to_u64 () , < U12 as Unsigned >:: to_u64 ()) ; }
}