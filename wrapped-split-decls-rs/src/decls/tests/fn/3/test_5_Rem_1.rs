use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_5_Rem_1 () { type A = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; type B = UInt < UTerm , B1 > ; type U0 = UTerm ; # [allow (non_camel_case_types)] type U5RemU1 = < < A as Rem < B > > :: Output as Same < U0 > > :: Output ; assert_eq ! (< U5RemU1 as Unsigned >:: to_u64 () , < U0 as Unsigned >:: to_u64 ()) ; }
}