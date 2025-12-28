use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_2_Mul_5 () { type A = UInt < UInt < UTerm , B1 > , B0 > ; type B = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; type U10 = UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > , B0 > ; # [allow (non_camel_case_types)] type U2MulU5 = < < A as Mul < B > > :: Output as Same < U10 > > :: Output ; assert_eq ! (< U2MulU5 as Unsigned >:: to_u64 () , < U10 as Unsigned >:: to_u64 ()) ; }
}