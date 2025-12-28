use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_3_Mul_3 () { type A = UInt < UInt < UTerm , B1 > , B1 > ; type B = UInt < UInt < UTerm , B1 > , B1 > ; type U9 = UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > , B1 > ; # [allow (non_camel_case_types)] type U3MulU3 = < < A as Mul < B > > :: Output as Same < U9 > > :: Output ; assert_eq ! (< U3MulU3 as Unsigned >:: to_u64 () , < U9 as Unsigned >:: to_u64 ()) ; }
}