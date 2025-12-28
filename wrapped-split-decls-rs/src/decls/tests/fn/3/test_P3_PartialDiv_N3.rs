use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P3_PartialDiv_N3 () { type A = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type N1 = NInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type P3PartialDivN3 = < < A as PartialDiv < B > > :: Output as Same < N1 > > :: Output ; assert_eq ! (< P3PartialDivN3 as Integer >:: to_i64 () , < N1 as Integer >:: to_i64 ()) ; }