use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N1_PartialDiv_P1 () { type A = NInt < UInt < UTerm , B1 > > ; type B = PInt < UInt < UTerm , B1 > > ; type N1 = NInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type N1PartialDivP1 = < < A as PartialDiv < B > > :: Output as Same < N1 > > :: Output ; assert_eq ! (< N1PartialDivP1 as Integer >:: to_i64 () , < N1 as Integer >:: to_i64 ()) ; }