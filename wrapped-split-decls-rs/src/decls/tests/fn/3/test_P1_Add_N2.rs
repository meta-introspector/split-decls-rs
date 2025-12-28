use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P1_Add_N2 () { type A = PInt < UInt < UTerm , B1 > > ; type B = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type N1 = NInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type P1AddN2 = < < A as Add < B > > :: Output as Same < N1 > > :: Output ; assert_eq ! (< P1AddN2 as Integer >:: to_i64 () , < N1 as Integer >:: to_i64 ()) ; }
}