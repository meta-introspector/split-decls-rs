use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P5_Mul_N3 () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type B = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type N15 = NInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B1 > , B1 > > ; # [allow (non_camel_case_types)] type P5MulN3 = < < A as Mul < B > > :: Output as Same < N15 > > :: Output ; assert_eq ! (< P5MulN3 as Integer >:: to_i64 () , < N15 as Integer >:: to_i64 ()) ; }
}