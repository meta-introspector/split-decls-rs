use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone , Serialize , Deserialize)] pub struct AstProbe { pub name : String , pub node_type : AstNodeType , pub filter : ProbeFilter , pub action : ProbeAction , pub enabled : bool , pub priority : u32 , }