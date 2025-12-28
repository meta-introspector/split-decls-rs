use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P3_Mul_N5 () { type A = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type N15 = NInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B1 > , B1 > > ; # [allow (non_camel_case_types)] type P3MulN5 = < < A as Mul < B > > :: Output as Same < N15 > > :: Output ; assert_eq ! (< P3MulN5 as Integer >:: to_i64 () , < N15 as Integer >:: to_i64 ()) ; }
}