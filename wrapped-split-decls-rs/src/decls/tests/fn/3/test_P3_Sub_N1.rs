use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P3_Sub_N1 () { type A = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = NInt < UInt < UTerm , B1 > > ; type P4 = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type P3SubN1 = < < A as Sub < B > > :: Output as Same < P4 > > :: Output ; assert_eq ! (< P3SubN1 as Integer >:: to_i64 () , < P4 as Integer >:: to_i64 ()) ; }
}