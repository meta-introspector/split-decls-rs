use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N4_Mul_N5 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type P20 = PInt < UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type N4MulN5 = < < A as Mul < B > > :: Output as Same < P20 > > :: Output ; assert_eq ! (< N4MulN5 as Integer >:: to_i64 () , < P20 as Integer >:: to_i64 ()) ; }
}