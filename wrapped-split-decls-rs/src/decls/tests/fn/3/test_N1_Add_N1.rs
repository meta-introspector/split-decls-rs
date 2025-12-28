use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N1_Add_N1 () { type A = NInt < UInt < UTerm , B1 > > ; type B = NInt < UInt < UTerm , B1 > > ; type N2 = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; # [allow (non_camel_case_types)] type N1AddN1 = < < A as Add < B > > :: Output as Same < N2 > > :: Output ; assert_eq ! (< N1AddN1 as Integer >:: to_i64 () , < N2 as Integer >:: to_i64 ()) ; }
}