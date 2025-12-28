use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P5_Add__0 () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type B = Z0 ; type P5 = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; # [allow (non_camel_case_types)] type P5Add_0 = < < A as Add < B > > :: Output as Same < P5 > > :: Output ; assert_eq ! (< P5Add_0 as Integer >:: to_i64 () , < P5 as Integer >:: to_i64 ()) ; }