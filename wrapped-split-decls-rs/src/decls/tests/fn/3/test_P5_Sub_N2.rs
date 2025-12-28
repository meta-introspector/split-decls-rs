use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P5_Sub_N2 () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type B = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type P7 = PInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B1 > > ; # [allow (non_camel_case_types)] type P5SubN2 = < < A as Sub < B > > :: Output as Same < P7 > > :: Output ; assert_eq ! (< P5SubN2 as Integer >:: to_i64 () , < P7 as Integer >:: to_i64 ()) ; }
}