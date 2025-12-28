use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_0_BitXor_3 () { type A = UTerm ; type B = UInt < UInt < UTerm , B1 > , B1 > ; type U3 = UInt < UInt < UTerm , B1 > , B1 > ; # [allow (non_camel_case_types)] type U0BitXorU3 = < < A as BitXor < B > > :: Output as Same < U3 > > :: Output ; assert_eq ! (< U0BitXorU3 as Unsigned >:: to_u64 () , < U3 as Unsigned >:: to_u64 ()) ; }
}