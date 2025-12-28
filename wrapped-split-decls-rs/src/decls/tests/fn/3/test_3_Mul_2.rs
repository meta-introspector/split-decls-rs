use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_3_Mul_2 () { type A = UInt < UInt < UTerm , B1 > , B1 > ; type B = UInt < UInt < UTerm , B1 > , B0 > ; type U6 = UInt < UInt < UInt < UTerm , B1 > , B1 > , B0 > ; # [allow (non_camel_case_types)] type U3MulU2 = < < A as Mul < B > > :: Output as Same < U6 > > :: Output ; assert_eq ! (< U3MulU2 as Unsigned >:: to_u64 () , < U6 as Unsigned >:: to_u64 ()) ; }
}