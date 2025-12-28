use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P1_Sub_P2 () { type A = PInt < UInt < UTerm , B1 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type N1 = NInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type P1SubP2 = < < A as Sub < B > > :: Output as Same < N1 > > :: Output ; assert_eq ! (< P1SubP2 as Integer >:: to_i64 () , < N1 as Integer >:: to_i64 ()) ; }
}