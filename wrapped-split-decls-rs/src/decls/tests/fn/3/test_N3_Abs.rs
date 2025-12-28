use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N3_Abs () { type A = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type P3 = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; # [allow (non_camel_case_types)] type AbsN3 = < < A as Abs > :: Output as Same < P3 > > :: Output ; assert_eq ! (< AbsN3 as Integer >:: to_i64 () , < P3 as Integer >:: to_i64 ()) ; }