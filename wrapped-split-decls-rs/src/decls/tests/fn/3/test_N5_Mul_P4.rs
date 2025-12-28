use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N5_Mul_P4 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type N20 = NInt < UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type N5MulP4 = < < A as Mul < B > > :: Output as Same < N20 > > :: Output ; assert_eq ! (< N5MulP4 as Integer >:: to_i64 () , < N20 as Integer >:: to_i64 ()) ; }
}