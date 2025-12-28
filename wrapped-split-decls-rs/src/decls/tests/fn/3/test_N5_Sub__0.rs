use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N5_Sub__0 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type B = Z0 ; type N5 = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; # [allow (non_camel_case_types)] type N5Sub_0 = < < A as Sub < B > > :: Output as Same < N5 > > :: Output ; assert_eq ! (< N5Sub_0 as Integer >:: to_i64 () , < N5 as Integer >:: to_i64 ()) ; }
}