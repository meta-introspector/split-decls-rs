use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_P4_Sub_N4");
# [test] # [allow (non_snake_case)] fn test_P4_Sub_N4 () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type P8 = PInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type P4SubN4 = < < A as Sub < B > > :: Output as Same < P8 > > :: Output ; assert_eq ! (< P4SubN4 as Integer >:: to_i64 () , < P8 as Integer >:: to_i64 ()) ; }
}