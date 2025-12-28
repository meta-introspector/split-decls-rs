use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P4_Mul_P4 () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type P16 = PInt < UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type P4MulP4 = < < A as Mul < B > > :: Output as Same < P16 > > :: Output ; assert_eq ! (< P4MulP4 as Integer >:: to_i64 () , < P16 as Integer >:: to_i64 ()) ; }
}