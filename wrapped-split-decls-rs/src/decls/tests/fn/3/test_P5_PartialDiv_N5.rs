use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P5_PartialDiv_N5 () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type N1 = NInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type P5PartialDivN5 = < < A as PartialDiv < B > > :: Output as Same < N1 > > :: Output ; assert_eq ! (< P5PartialDivN5 as Integer >:: to_i64 () , < N1 as Integer >:: to_i64 ()) ; }
}