use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_2_Mul_2 () { type A = UInt < UInt < UTerm , B1 > , B0 > ; type B = UInt < UInt < UTerm , B1 > , B0 > ; type U4 = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; # [allow (non_camel_case_types)] type U2MulU2 = < < A as Mul < B > > :: Output as Same < U4 > > :: Output ; assert_eq ! (< U2MulU2 as Unsigned >:: to_u64 () , < U4 as Unsigned >:: to_u64 ()) ; }
}