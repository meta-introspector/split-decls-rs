use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N1_Sub_P4 () { type A = NInt < UInt < UTerm , B1 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type N5 = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; # [allow (non_camel_case_types)] type N1SubP4 = < < A as Sub < B > > :: Output as Same < N5 > > :: Output ; assert_eq ! (< N1SubP4 as Integer >:: to_i64 () , < N5 as Integer >:: to_i64 ()) ; }
}