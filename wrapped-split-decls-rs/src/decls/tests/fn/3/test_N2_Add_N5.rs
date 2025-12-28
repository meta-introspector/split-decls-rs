use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N2_Add_N5 () { type A = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type N7 = NInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B1 > > ; # [allow (non_camel_case_types)] type N2AddN5 = < < A as Add < B > > :: Output as Same < N7 > > :: Output ; assert_eq ! (< N2AddN5 as Integer >:: to_i64 () , < N7 as Integer >:: to_i64 ()) ; }
}