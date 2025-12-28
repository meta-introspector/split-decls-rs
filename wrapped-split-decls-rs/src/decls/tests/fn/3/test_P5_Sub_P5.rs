use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P5_Sub_P5 () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type _0 = Z0 ; # [allow (non_camel_case_types)] type P5SubP5 = < < A as Sub < B > > :: Output as Same < _0 > > :: Output ; assert_eq ! (< P5SubP5 as Integer >:: to_i64 () , < _0 as Integer >:: to_i64 ()) ; }