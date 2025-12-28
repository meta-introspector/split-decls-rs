use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N5_Sub_P3 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type N8 = NInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type N5SubP3 = < < A as Sub < B > > :: Output as Same < N8 > > :: Output ; assert_eq ! (< N5SubP3 as Integer >:: to_i64 () , < N8 as Integer >:: to_i64 ()) ; }