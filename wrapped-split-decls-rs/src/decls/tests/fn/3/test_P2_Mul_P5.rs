use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P2_Mul_P5 () { type A = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type P10 = PInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > , B0 > > ; # [allow (non_camel_case_types)] type P2MulP5 = < < A as Mul < B > > :: Output as Same < P10 > > :: Output ; assert_eq ! (< P2MulP5 as Integer >:: to_i64 () , < P10 as Integer >:: to_i64 ()) ; }
}