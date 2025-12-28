use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P4_Sub_N3 () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type P7 = PInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B1 > > ; # [allow (non_camel_case_types)] type P4SubN3 = < < A as Sub < B > > :: Output as Same < P7 > > :: Output ; assert_eq ! (< P4SubN3 as Integer >:: to_i64 () , < P7 as Integer >:: to_i64 ()) ; }