use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N2_Sub_P3 () { type A = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type N5 = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; # [allow (non_camel_case_types)] type N2SubP3 = < < A as Sub < B > > :: Output as Same < N5 > > :: Output ; assert_eq ! (< N2SubP3 as Integer >:: to_i64 () , < N5 as Integer >:: to_i64 ()) ; }
}