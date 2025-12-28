use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N5_PartialDiv_P1 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type B = PInt < UInt < UTerm , B1 > > ; type N5 = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; # [allow (non_camel_case_types)] type N5PartialDivP1 = < < A as PartialDiv < B > > :: Output as Same < N5 > > :: Output ; assert_eq ! (< N5PartialDivP1 as Integer >:: to_i64 () , < N5 as Integer >:: to_i64 ()) ; }