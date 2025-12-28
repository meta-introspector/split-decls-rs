use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P2_Mul_N5 () { type A = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type N10 = NInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > , B0 > > ; # [allow (non_camel_case_types)] type P2MulN5 = < < A as Mul < B > > :: Output as Same < N10 > > :: Output ; assert_eq ! (< P2MulN5 as Integer >:: to_i64 () , < N10 as Integer >:: to_i64 ()) ; }
}