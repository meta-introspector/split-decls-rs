use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N1_Sub_N3 () { type A = NInt < UInt < UTerm , B1 > > ; type B = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type P2 = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; # [allow (non_camel_case_types)] type N1SubN3 = < < A as Sub < B > > :: Output as Same < P2 > > :: Output ; assert_eq ! (< N1SubN3 as Integer >:: to_i64 () , < P2 as Integer >:: to_i64 ()) ; }
}