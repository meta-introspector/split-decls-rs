use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P2_Abs () { type A = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type P2 = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; # [allow (non_camel_case_types)] type AbsP2 = < < A as Abs > :: Output as Same < P2 > > :: Output ; assert_eq ! (< AbsP2 as Integer >:: to_i64 () , < P2 as Integer >:: to_i64 ()) ; }
}