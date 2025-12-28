use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P4_Min_P3 () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type P3 = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; # [allow (non_camel_case_types)] type P4MinP3 = < < A as Min < B > > :: Output as Same < P3 > > :: Output ; assert_eq ! (< P4MinP3 as Integer >:: to_i64 () , < P3 as Integer >:: to_i64 ()) ; }
}