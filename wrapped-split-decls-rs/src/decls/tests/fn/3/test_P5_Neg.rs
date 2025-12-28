use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P5_Neg () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type N5 = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; # [allow (non_camel_case_types)] type NegP5 = < < A as Neg > :: Output as Same < N5 > > :: Output ; assert_eq ! (< NegP5 as Integer >:: to_i64 () , < N5 as Integer >:: to_i64 ()) ; }
}