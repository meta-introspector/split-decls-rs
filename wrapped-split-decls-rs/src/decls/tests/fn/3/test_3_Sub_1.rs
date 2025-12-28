use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_3_Sub_1 () { type A = UInt < UInt < UTerm , B1 > , B1 > ; type B = UInt < UTerm , B1 > ; type U2 = UInt < UInt < UTerm , B1 > , B0 > ; # [allow (non_camel_case_types)] type U3SubU1 = < < A as Sub < B > > :: Output as Same < U2 > > :: Output ; assert_eq ! (< U3SubU1 as Unsigned >:: to_u64 () , < U2 as Unsigned >:: to_u64 ()) ; }
}