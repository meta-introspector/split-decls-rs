use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P3_Neg () { type A = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type N3 = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; # [allow (non_camel_case_types)] type NegP3 = < < A as Neg > :: Output as Same < N3 > > :: Output ; assert_eq ! (< NegP3 as Integer >:: to_i64 () , < N3 as Integer >:: to_i64 ()) ; }