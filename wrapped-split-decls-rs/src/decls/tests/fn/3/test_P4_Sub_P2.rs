use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P4_Sub_P2 () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type P2 = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; # [allow (non_camel_case_types)] type P4SubP2 = < < A as Sub < B > > :: Output as Same < P2 > > :: Output ; assert_eq ! (< P4SubP2 as Integer >:: to_i64 () , < P2 as Integer >:: to_i64 ()) ; }