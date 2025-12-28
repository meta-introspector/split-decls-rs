use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N2_Mul_P1 () { type A = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = PInt < UInt < UTerm , B1 > > ; type N2 = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; # [allow (non_camel_case_types)] type N2MulP1 = < < A as Mul < B > > :: Output as Same < N2 > > :: Output ; assert_eq ! (< N2MulP1 as Integer >:: to_i64 () , < N2 as Integer >:: to_i64 ()) ; }
}