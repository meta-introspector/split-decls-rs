use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P1_Sub_N4 () { type A = PInt < UInt < UTerm , B1 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type P5 = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; # [allow (non_camel_case_types)] type P1SubN4 = < < A as Sub < B > > :: Output as Same < P5 > > :: Output ; assert_eq ! (< P1SubN4 as Integer >:: to_i64 () , < P5 as Integer >:: to_i64 ()) ; }
}