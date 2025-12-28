use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P1_Sub__0 () { type A = PInt < UInt < UTerm , B1 > > ; type B = Z0 ; type P1 = PInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type P1Sub_0 = < < A as Sub < B > > :: Output as Same < P1 > > :: Output ; assert_eq ! (< P1Sub_0 as Integer >:: to_i64 () , < P1 as Integer >:: to_i64 ()) ; }