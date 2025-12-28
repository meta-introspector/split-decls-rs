use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P2_Neg () { type A = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type N2 = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; # [allow (non_camel_case_types)] type NegP2 = < < A as Neg > :: Output as Same < N2 > > :: Output ; assert_eq ! (< NegP2 as Integer >:: to_i64 () , < N2 as Integer >:: to_i64 ()) ; }
}