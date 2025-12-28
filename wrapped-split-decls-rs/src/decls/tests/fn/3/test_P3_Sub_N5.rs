use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P3_Sub_N5 () { type A = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type P8 = PInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type P3SubN5 = < < A as Sub < B > > :: Output as Same < P8 > > :: Output ; assert_eq ! (< P3SubN5 as Integer >:: to_i64 () , < P8 as Integer >:: to_i64 ()) ; }
}