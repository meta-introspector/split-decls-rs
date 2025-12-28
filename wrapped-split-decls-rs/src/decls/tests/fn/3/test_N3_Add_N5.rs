use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N3_Add_N5 () { type A = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type N8 = NInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type N3AddN5 = < < A as Add < B > > :: Output as Same < N8 > > :: Output ; assert_eq ! (< N3AddN5 as Integer >:: to_i64 () , < N8 as Integer >:: to_i64 ()) ; }
}