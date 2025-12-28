use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P5_Sub_N1 () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type B = NInt < UInt < UTerm , B1 > > ; type P6 = PInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B0 > > ; # [allow (non_camel_case_types)] type P5SubN1 = < < A as Sub < B > > :: Output as Same < P6 > > :: Output ; assert_eq ! (< P5SubN1 as Integer >:: to_i64 () , < P6 as Integer >:: to_i64 ()) ; }