use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N2_Max_N1 () { type A = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = NInt < UInt < UTerm , B1 > > ; type N1 = NInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type N2MaxN1 = < < A as Max < B > > :: Output as Same < N1 > > :: Output ; assert_eq ! (< N2MaxN1 as Integer >:: to_i64 () , < N1 as Integer >:: to_i64 ()) ; }