use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_4_BitAnd_3 () { type A = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; type B = UInt < UInt < UTerm , B1 > , B1 > ; type U0 = UTerm ; # [allow (non_camel_case_types)] type U4BitAndU3 = < < A as BitAnd < B > > :: Output as Same < U0 > > :: Output ; assert_eq ! (< U4BitAndU3 as Unsigned >:: to_u64 () , < U0 as Unsigned >:: to_u64 ()) ; }
}