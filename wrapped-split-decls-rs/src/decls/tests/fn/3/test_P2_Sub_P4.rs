use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P2_Sub_P4 () { type A = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type N2 = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; # [allow (non_camel_case_types)] type P2SubP4 = < < A as Sub < B > > :: Output as Same < N2 > > :: Output ; assert_eq ! (< P2SubP4 as Integer >:: to_i64 () , < N2 as Integer >:: to_i64 ()) ; }