use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P4_Neg () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type N4 = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type NegP4 = < < A as Neg > :: Output as Same < N4 > > :: Output ; assert_eq ! (< NegP4 as Integer >:: to_i64 () , < N4 as Integer >:: to_i64 ()) ; }
}