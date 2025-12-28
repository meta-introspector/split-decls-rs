use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_5_BitXor_3 () { type A = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; type B = UInt < UInt < UTerm , B1 > , B1 > ; type U6 = UInt < UInt < UInt < UTerm , B1 > , B1 > , B0 > ; # [allow (non_camel_case_types)] type U5BitXorU3 = < < A as BitXor < B > > :: Output as Same < U6 > > :: Output ; assert_eq ! (< U5BitXorU3 as Unsigned >:: to_u64 () , < U6 as Unsigned >:: to_u64 ()) ; }
}