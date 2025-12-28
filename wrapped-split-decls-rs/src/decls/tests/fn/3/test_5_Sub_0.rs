use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_5_Sub_0 () { type A = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; type B = UTerm ; type U5 = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; # [allow (non_camel_case_types)] type U5SubU0 = < < A as Sub < B > > :: Output as Same < U5 > > :: Output ; assert_eq ! (< U5SubU0 as Unsigned >:: to_u64 () , < U5 as Unsigned >:: to_u64 ()) ; }
}