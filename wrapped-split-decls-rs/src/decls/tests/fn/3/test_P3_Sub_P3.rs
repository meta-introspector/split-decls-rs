use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P3_Sub_P3 () { type A = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type _0 = Z0 ; # [allow (non_camel_case_types)] type P3SubP3 = < < A as Sub < B > > :: Output as Same < _0 > > :: Output ; assert_eq ! (< P3SubP3 as Integer >:: to_i64 () , < _0 as Integer >:: to_i64 ()) ; }