use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N4_Add_N2 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type N6 = NInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B0 > > ; # [allow (non_camel_case_types)] type N4AddN2 = < < A as Add < B > > :: Output as Same < N6 > > :: Output ; assert_eq ! (< N4AddN2 as Integer >:: to_i64 () , < N6 as Integer >:: to_i64 ()) ; }
}