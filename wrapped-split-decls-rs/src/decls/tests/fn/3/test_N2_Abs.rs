use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N2_Abs () { type A = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type P2 = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; # [allow (non_camel_case_types)] type AbsN2 = < < A as Abs > :: Output as Same < P2 > > :: Output ; assert_eq ! (< AbsN2 as Integer >:: to_i64 () , < P2 as Integer >:: to_i64 ()) ; }