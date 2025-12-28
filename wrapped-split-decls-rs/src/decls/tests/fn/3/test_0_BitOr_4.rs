use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_0_BitOr_4 () { type A = UTerm ; type B = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; type U4 = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; # [allow (non_camel_case_types)] type U0BitOrU4 = < < A as BitOr < B > > :: Output as Same < U4 > > :: Output ; assert_eq ! (< U0BitOrU4 as Unsigned >:: to_u64 () , < U4 as Unsigned >:: to_u64 ()) ; }
}