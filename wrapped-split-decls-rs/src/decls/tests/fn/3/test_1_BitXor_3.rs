use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_1_BitXor_3 () { type A = UInt < UTerm , B1 > ; type B = UInt < UInt < UTerm , B1 > , B1 > ; type U2 = UInt < UInt < UTerm , B1 > , B0 > ; # [allow (non_camel_case_types)] type U1BitXorU3 = < < A as BitXor < B > > :: Output as Same < U2 > > :: Output ; assert_eq ! (< U1BitXorU3 as Unsigned >:: to_u64 () , < U2 as Unsigned >:: to_u64 ()) ; }
}