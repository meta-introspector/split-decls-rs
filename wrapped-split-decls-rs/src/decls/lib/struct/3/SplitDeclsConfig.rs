use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone , Default)] pub struct SplitDeclsConfig { pub patches : Option < HashMap < String , Vec < String > > > , pub string_replacements : Option < Vec < StringReplacement > > , pub custom_prelude_overlay : Option < String > , pub active_overlay_modules : Vec < String > , pub crates_io_patches : HashMap < String , String > , }