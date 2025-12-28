use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N4_Mul_N4 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type P16 = PInt < UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type N4MulN4 = < < A as Mul < B > > :: Output as Same < P16 > > :: Output ; assert_eq ! (< N4MulN4 as Integer >:: to_i64 () , < P16 as Integer >:: to_i64 ()) ; }
}