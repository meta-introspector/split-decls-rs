use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_1_BitOr_5 () { type A = UInt < UTerm , B1 > ; type B = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; type U5 = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; # [allow (non_camel_case_types)] type U1BitOrU5 = < < A as BitOr < B > > :: Output as Same < U5 > > :: Output ; assert_eq ! (< U1BitOrU5 as Unsigned >:: to_u64 () , < U5 as Unsigned >:: to_u64 ()) ; }
}