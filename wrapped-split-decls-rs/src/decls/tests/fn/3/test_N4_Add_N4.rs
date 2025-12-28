use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N4_Add_N4 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type N8 = NInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type N4AddN4 = < < A as Add < B > > :: Output as Same < N8 > > :: Output ; assert_eq ! (< N4AddN4 as Integer >:: to_i64 () , < N8 as Integer >:: to_i64 ()) ; }
}