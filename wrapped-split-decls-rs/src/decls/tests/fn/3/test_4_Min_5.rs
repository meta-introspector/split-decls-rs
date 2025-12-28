use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_4_Min_5 () { type A = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; type B = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; type U4 = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; # [allow (non_camel_case_types)] type U4MinU5 = < < A as Min < B > > :: Output as Same < U4 > > :: Output ; assert_eq ! (< U4MinU5 as Unsigned >:: to_u64 () , < U4 as Unsigned >:: to_u64 ()) ; }
}