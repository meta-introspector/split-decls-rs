use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P4_PartialDiv_N1 () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = NInt < UInt < UTerm , B1 > > ; type N4 = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type P4PartialDivN1 = < < A as PartialDiv < B > > :: Output as Same < N4 > > :: Output ; assert_eq ! (< P4PartialDivN1 as Integer >:: to_i64 () , < N4 as Integer >:: to_i64 ()) ; }
}