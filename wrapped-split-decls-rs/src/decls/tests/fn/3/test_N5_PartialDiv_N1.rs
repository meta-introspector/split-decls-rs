use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N5_PartialDiv_N1 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type B = NInt < UInt < UTerm , B1 > > ; type P5 = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; # [allow (non_camel_case_types)] type N5PartialDivN1 = < < A as PartialDiv < B > > :: Output as Same < P5 > > :: Output ; assert_eq ! (< N5PartialDivN1 as Integer >:: to_i64 () , < P5 as Integer >:: to_i64 ()) ; }