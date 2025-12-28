use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N1_Min_N5 () { type A = NInt < UInt < UTerm , B1 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type N5 = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; # [allow (non_camel_case_types)] type N1MinN5 = < < A as Min < B > > :: Output as Same < N5 > > :: Output ; assert_eq ! (< N1MinN5 as Integer >:: to_i64 () , < N5 as Integer >:: to_i64 ()) ; }