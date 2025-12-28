use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_3_BitXor_4 () { type A = UInt < UInt < UTerm , B1 > , B1 > ; type B = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; type U7 = UInt < UInt < UInt < UTerm , B1 > , B1 > , B1 > ; # [allow (non_camel_case_types)] type U3BitXorU4 = < < A as BitXor < B > > :: Output as Same < U7 > > :: Output ; assert_eq ! (< U3BitXorU4 as Unsigned >:: to_u64 () , < U7 as Unsigned >:: to_u64 ()) ; }
}