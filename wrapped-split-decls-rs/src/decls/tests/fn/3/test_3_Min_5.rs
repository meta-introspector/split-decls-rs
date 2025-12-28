use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_3_Min_5 () { type A = UInt < UInt < UTerm , B1 > , B1 > ; type B = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; type U3 = UInt < UInt < UTerm , B1 > , B1 > ; # [allow (non_camel_case_types)] type U3MinU5 = < < A as Min < B > > :: Output as Same < U3 > > :: Output ; assert_eq ! (< U3MinU5 as Unsigned >:: to_u64 () , < U3 as Unsigned >:: to_u64 ()) ; }
}