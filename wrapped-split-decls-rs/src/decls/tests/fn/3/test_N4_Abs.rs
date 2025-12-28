use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N4_Abs () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type P4 = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type AbsN4 = < < A as Abs > :: Output as Same < P4 > > :: Output ; assert_eq ! (< AbsN4 as Integer >:: to_i64 () , < P4 as Integer >:: to_i64 ()) ; }