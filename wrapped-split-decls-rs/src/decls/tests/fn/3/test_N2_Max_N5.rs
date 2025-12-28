use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N2_Max_N5 () { type A = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type N2 = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; # [allow (non_camel_case_types)] type N2MaxN5 = < < A as Max < B > > :: Output as Same < N2 > > :: Output ; assert_eq ! (< N2MaxN5 as Integer >:: to_i64 () , < N2 as Integer >:: to_i64 ()) ; }