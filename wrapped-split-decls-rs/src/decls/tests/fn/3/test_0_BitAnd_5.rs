use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_0_BitAnd_5 () { type A = UTerm ; type B = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; type U0 = UTerm ; # [allow (non_camel_case_types)] type U0BitAndU5 = < < A as BitAnd < B > > :: Output as Same < U0 > > :: Output ; assert_eq ! (< U0BitAndU5 as Unsigned >:: to_u64 () , < U0 as Unsigned >:: to_u64 ()) ; }
}