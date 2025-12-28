use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_4_Mul_5 () { type A = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; type B = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; type U20 = UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > , B0 > , B0 > ; # [allow (non_camel_case_types)] type U4MulU5 = < < A as Mul < B > > :: Output as Same < U20 > > :: Output ; assert_eq ! (< U4MulU5 as Unsigned >:: to_u64 () , < U20 as Unsigned >:: to_u64 ()) ; }
}