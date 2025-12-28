use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P3_Add_N5 () { type A = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type N2 = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; # [allow (non_camel_case_types)] type P3AddN5 = < < A as Add < B > > :: Output as Same < N2 > > :: Output ; assert_eq ! (< P3AddN5 as Integer >:: to_i64 () , < N2 as Integer >:: to_i64 ()) ; }
}