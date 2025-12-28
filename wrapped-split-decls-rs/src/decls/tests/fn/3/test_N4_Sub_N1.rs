use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N4_Sub_N1 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = NInt < UInt < UTerm , B1 > > ; type N3 = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; # [allow (non_camel_case_types)] type N4SubN1 = < < A as Sub < B > > :: Output as Same < N3 > > :: Output ; assert_eq ! (< N4SubN1 as Integer >:: to_i64 () , < N3 as Integer >:: to_i64 ()) ; }
}