use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_1_BitAnd_2 () { type A = UInt < UTerm , B1 > ; type B = UInt < UInt < UTerm , B1 > , B0 > ; type U0 = UTerm ; # [allow (non_camel_case_types)] type U1BitAndU2 = < < A as BitAnd < B > > :: Output as Same < U0 > > :: Output ; assert_eq ! (< U1BitAndU2 as Unsigned >:: to_u64 () , < U0 as Unsigned >:: to_u64 ()) ; }
}