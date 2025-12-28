use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P2_Sub__0 () { type A = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = Z0 ; type P2 = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; # [allow (non_camel_case_types)] type P2Sub_0 = < < A as Sub < B > > :: Output as Same < P2 > > :: Output ; assert_eq ! (< P2Sub_0 as Integer >:: to_i64 () , < P2 as Integer >:: to_i64 ()) ; }
}