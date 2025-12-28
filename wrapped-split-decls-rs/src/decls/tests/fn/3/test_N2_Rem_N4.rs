use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N2_Rem_N4 () { type A = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type N2 = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; # [allow (non_camel_case_types)] type N2RemN4 = < < A as Rem < B > > :: Output as Same < N2 > > :: Output ; assert_eq ! (< N2RemN4 as Integer >:: to_i64 () , < N2 as Integer >:: to_i64 ()) ; }
}