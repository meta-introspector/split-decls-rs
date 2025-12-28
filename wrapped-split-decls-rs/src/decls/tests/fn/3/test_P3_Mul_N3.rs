use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P3_Mul_N3 () { type A = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type N9 = NInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > , B1 > > ; # [allow (non_camel_case_types)] type P3MulN3 = < < A as Mul < B > > :: Output as Same < N9 > > :: Output ; assert_eq ! (< P3MulN3 as Integer >:: to_i64 () , < N9 as Integer >:: to_i64 ()) ; }
}