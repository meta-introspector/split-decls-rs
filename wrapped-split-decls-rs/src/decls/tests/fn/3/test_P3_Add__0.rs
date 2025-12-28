use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P3_Add__0 () { type A = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = Z0 ; type P3 = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; # [allow (non_camel_case_types)] type P3Add_0 = < < A as Add < B > > :: Output as Same < P3 > > :: Output ; assert_eq ! (< P3Add_0 as Integer >:: to_i64 () , < P3 as Integer >:: to_i64 ()) ; }