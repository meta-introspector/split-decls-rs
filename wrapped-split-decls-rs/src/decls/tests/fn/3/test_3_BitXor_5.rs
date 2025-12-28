use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_3_BitXor_5 () { type A = UInt < UInt < UTerm , B1 > , B1 > ; type B = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; type U6 = UInt < UInt < UInt < UTerm , B1 > , B1 > , B0 > ; # [allow (non_camel_case_types)] type U3BitXorU5 = < < A as BitXor < B > > :: Output as Same < U6 > > :: Output ; assert_eq ! (< U3BitXorU5 as Unsigned >:: to_u64 () , < U6 as Unsigned >:: to_u64 ()) ; }
}