use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N1_Mul_P3 () { type A = NInt < UInt < UTerm , B1 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type N3 = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; # [allow (non_camel_case_types)] type N1MulP3 = < < A as Mul < B > > :: Output as Same < N3 > > :: Output ; assert_eq ! (< N1MulP3 as Integer >:: to_i64 () , < N3 as Integer >:: to_i64 ()) ; }
}