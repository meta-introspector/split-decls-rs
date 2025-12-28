use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N2_Mul_N3 () { type A = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type P6 = PInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B0 > > ; # [allow (non_camel_case_types)] type N2MulN3 = < < A as Mul < B > > :: Output as Same < P6 > > :: Output ; assert_eq ! (< N2MulN3 as Integer >:: to_i64 () , < P6 as Integer >:: to_i64 ()) ; }
}