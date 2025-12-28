use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_P3_Pow_P5");
# [test] # [allow (non_snake_case)] fn test_P3_Pow_P5 () { type A = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type P243 = PInt < UInt < UInt < UInt < UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B1 > , B1 > , B0 > , B0 > , B1 > , B1 > > ; # [allow (non_camel_case_types)] type P3PowP5 = < < A as Pow < B > > :: Output as Same < P243 > > :: Output ; assert_eq ! (< P3PowP5 as Integer >:: to_i64 () , < P243 as Integer >:: to_i64 ()) ; }
}