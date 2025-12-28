use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N4_PartialDiv_P4 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type N1 = NInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type N4PartialDivP4 = < < A as PartialDiv < B > > :: Output as Same < N1 > > :: Output ; assert_eq ! (< N4PartialDivP4 as Integer >:: to_i64 () , < N1 as Integer >:: to_i64 ()) ; }