use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N2_Mul_P4 () { type A = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type N8 = NInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type N2MulP4 = < < A as Mul < B > > :: Output as Same < N8 > > :: Output ; assert_eq ! (< N2MulP4 as Integer >:: to_i64 () , < N8 as Integer >:: to_i64 ()) ; }
}