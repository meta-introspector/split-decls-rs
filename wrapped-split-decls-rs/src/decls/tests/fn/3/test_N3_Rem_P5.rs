use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N3_Rem_P5 () { type A = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type N3 = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; # [allow (non_camel_case_types)] type N3RemP5 = < < A as Rem < B > > :: Output as Same < N3 > > :: Output ; assert_eq ! (< N3RemP5 as Integer >:: to_i64 () , < N3 as Integer >:: to_i64 ()) ; }
}