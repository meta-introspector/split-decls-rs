use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N3_Add_N3 () { type A = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type N6 = NInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B0 > > ; # [allow (non_camel_case_types)] type N3AddN3 = < < A as Add < B > > :: Output as Same < N6 > > :: Output ; assert_eq ! (< N3AddN3 as Integer >:: to_i64 () , < N6 as Integer >:: to_i64 ()) ; }
}