use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_5_Div_2 () { type A = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; type B = UInt < UInt < UTerm , B1 > , B0 > ; type U2 = UInt < UInt < UTerm , B1 > , B0 > ; # [allow (non_camel_case_types)] type U5DivU2 = < < A as Div < B > > :: Output as Same < U2 > > :: Output ; assert_eq ! (< U5DivU2 as Unsigned >:: to_u64 () , < U2 as Unsigned >:: to_u64 ()) ; }
}