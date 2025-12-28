use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N2_Sub_P5 () { type A = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type N7 = NInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B1 > > ; # [allow (non_camel_case_types)] type N2SubP5 = < < A as Sub < B > > :: Output as Same < N7 > > :: Output ; assert_eq ! (< N2SubP5 as Integer >:: to_i64 () , < N7 as Integer >:: to_i64 ()) ; }
}