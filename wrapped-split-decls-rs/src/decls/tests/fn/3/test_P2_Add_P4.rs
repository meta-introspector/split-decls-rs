use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P2_Add_P4 () { type A = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type P6 = PInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B0 > > ; # [allow (non_camel_case_types)] type P2AddP4 = < < A as Add < B > > :: Output as Same < P6 > > :: Output ; assert_eq ! (< P2AddP4 as Integer >:: to_i64 () , < P6 as Integer >:: to_i64 ()) ; }