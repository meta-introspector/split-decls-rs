use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N2_Sub_N5 () { type A = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type P3 = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; # [allow (non_camel_case_types)] type N2SubN5 = < < A as Sub < B > > :: Output as Same < P3 > > :: Output ; assert_eq ! (< N2SubN5 as Integer >:: to_i64 () , < P3 as Integer >:: to_i64 ()) ; }
}