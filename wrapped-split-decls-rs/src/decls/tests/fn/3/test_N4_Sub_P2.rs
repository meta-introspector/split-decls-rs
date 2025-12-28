use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N4_Sub_P2 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type N6 = NInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B0 > > ; # [allow (non_camel_case_types)] type N4SubP2 = < < A as Sub < B > > :: Output as Same < N6 > > :: Output ; assert_eq ! (< N4SubP2 as Integer >:: to_i64 () , < N6 as Integer >:: to_i64 ()) ; }
}