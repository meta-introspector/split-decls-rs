use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N1_Mul_N5 () { type A = NInt < UInt < UTerm , B1 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type P5 = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; # [allow (non_camel_case_types)] type N1MulN5 = < < A as Mul < B > > :: Output as Same < P5 > > :: Output ; assert_eq ! (< N1MulN5 as Integer >:: to_i64 () , < P5 as Integer >:: to_i64 ()) ; }
}