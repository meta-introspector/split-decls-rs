use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_3_Mul_5 () { type A = UInt < UInt < UTerm , B1 > , B1 > ; type B = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; type U15 = UInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B1 > , B1 > ; # [allow (non_camel_case_types)] type U3MulU5 = < < A as Mul < B > > :: Output as Same < U15 > > :: Output ; assert_eq ! (< U3MulU5 as Unsigned >:: to_u64 () , < U15 as Unsigned >:: to_u64 ()) ; }
}