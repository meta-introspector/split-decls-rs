use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Serialize , Deserialize)] pub struct KIndex { pub nodes : HashMap < String , KNode > , pub levels : HashMap < u8 , Vec < String > > , }