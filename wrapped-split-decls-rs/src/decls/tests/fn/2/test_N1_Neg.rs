use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N1_Neg () { type A = NInt < UInt < UTerm , B1 > > ; type P1 = PInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type NegN1 = < < A as Neg > :: Output as Same < P1 > > :: Output ; assert_eq ! (< NegN1 as Integer >:: to_i64 () , < P1 as Integer >:: to_i64 ()) ; }