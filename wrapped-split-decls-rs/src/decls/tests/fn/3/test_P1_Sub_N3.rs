use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P1_Sub_N3 () { type A = PInt < UInt < UTerm , B1 > > ; type B = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type P4 = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type P1SubN3 = < < A as Sub < B > > :: Output as Same < P4 > > :: Output ; assert_eq ! (< P1SubN3 as Integer >:: to_i64 () , < P4 as Integer >:: to_i64 ()) ; }
}