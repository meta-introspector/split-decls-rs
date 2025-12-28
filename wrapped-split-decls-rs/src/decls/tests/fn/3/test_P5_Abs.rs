use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P5_Abs () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type P5 = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; # [allow (non_camel_case_types)] type AbsP5 = < < A as Abs > :: Output as Same < P5 > > :: Output ; assert_eq ! (< AbsP5 as Integer >:: to_i64 () , < P5 as Integer >:: to_i64 ()) ; }
}