use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N4_Mul_P3 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type N12 = NInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type N4MulP3 = < < A as Mul < B > > :: Output as Same < N12 > > :: Output ; assert_eq ! (< N4MulP3 as Integer >:: to_i64 () , < N12 as Integer >:: to_i64 ()) ; }
}