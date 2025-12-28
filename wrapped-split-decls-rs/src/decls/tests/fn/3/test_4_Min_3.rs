use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_4_Min_3 () { type A = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; type B = UInt < UInt < UTerm , B1 > , B1 > ; type U3 = UInt < UInt < UTerm , B1 > , B1 > ; # [allow (non_camel_case_types)] type U4MinU3 = < < A as Min < B > > :: Output as Same < U3 > > :: Output ; assert_eq ! (< U4MinU3 as Unsigned >:: to_u64 () , < U3 as Unsigned >:: to_u64 ()) ; }
}