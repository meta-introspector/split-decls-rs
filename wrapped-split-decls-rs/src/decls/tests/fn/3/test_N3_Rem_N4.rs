use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N3_Rem_N4 () { type A = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type N3 = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; # [allow (non_camel_case_types)] type N3RemN4 = < < A as Rem < B > > :: Output as Same < N3 > > :: Output ; assert_eq ! (< N3RemN4 as Integer >:: to_i64 () , < N3 as Integer >:: to_i64 ()) ; }
}