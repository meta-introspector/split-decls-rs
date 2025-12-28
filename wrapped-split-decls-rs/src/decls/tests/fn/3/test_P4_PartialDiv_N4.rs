use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P4_PartialDiv_N4 () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type N1 = NInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type P4PartialDivN4 = < < A as PartialDiv < B > > :: Output as Same < N1 > > :: Output ; assert_eq ! (< P4PartialDivN4 as Integer >:: to_i64 () , < N1 as Integer >:: to_i64 ()) ; }
}