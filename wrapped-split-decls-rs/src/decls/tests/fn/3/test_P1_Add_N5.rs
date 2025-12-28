use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P1_Add_N5 () { type A = PInt < UInt < UTerm , B1 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type N4 = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type P1AddN5 = < < A as Add < B > > :: Output as Same < N4 > > :: Output ; assert_eq ! (< P1AddN5 as Integer >:: to_i64 () , < N4 as Integer >:: to_i64 ()) ; }
}