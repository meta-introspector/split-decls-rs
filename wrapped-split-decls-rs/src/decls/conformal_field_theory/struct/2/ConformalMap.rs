use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Clone , Serialize , Deserialize)] pub struct ConformalMap { pub phi_8d : Level8DPoint , pub c1_to_n1 : HashMap < String , String > , pub n1_to_c2 : HashMap < String , String > , pub angle_preservation : Vec < AnglePreservation > , }
}