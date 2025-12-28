use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P4_Min_P2 () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type P2 = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; # [allow (non_camel_case_types)] type P4MinP2 = < < A as Min < B > > :: Output as Same < P2 > > :: Output ; assert_eq ! (< P4MinP2 as Integer >:: to_i64 () , < P2 as Integer >:: to_i64 ()) ; }
}