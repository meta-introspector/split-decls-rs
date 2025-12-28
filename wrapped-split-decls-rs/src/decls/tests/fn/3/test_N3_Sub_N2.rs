use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N3_Sub_N2 () { type A = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type N1 = NInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type N3SubN2 = < < A as Sub < B > > :: Output as Same < N1 > > :: Output ; assert_eq ! (< N3SubN2 as Integer >:: to_i64 () , < N1 as Integer >:: to_i64 ()) ; }