use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_2_Mul_3 () { type A = UInt < UInt < UTerm , B1 > , B0 > ; type B = UInt < UInt < UTerm , B1 > , B1 > ; type U6 = UInt < UInt < UInt < UTerm , B1 > , B1 > , B0 > ; # [allow (non_camel_case_types)] type U2MulU3 = < < A as Mul < B > > :: Output as Same < U6 > > :: Output ; assert_eq ! (< U2MulU3 as Unsigned >:: to_u64 () , < U6 as Unsigned >:: to_u64 ()) ; }
}