use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_1_Div_1 () { type A = UInt < UTerm , B1 > ; type B = UInt < UTerm , B1 > ; type U1 = UInt < UTerm , B1 > ; # [allow (non_camel_case_types)] type U1DivU1 = < < A as Div < B > > :: Output as Same < U1 > > :: Output ; assert_eq ! (< U1DivU1 as Unsigned >:: to_u64 () , < U1 as Unsigned >:: to_u64 ()) ; }
}