use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P2_Sub_P5 () { type A = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type N3 = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; # [allow (non_camel_case_types)] type P2SubP5 = < < A as Sub < B > > :: Output as Same < N3 > > :: Output ; assert_eq ! (< P2SubP5 as Integer >:: to_i64 () , < N3 as Integer >:: to_i64 ()) ; }
}