use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , PartialEq , Eq , Debug)] pub enum RenderOption { NoEdgeLabels , NoNodeLabels , NoEdgeStyles , NoNodeStyles , Fontname (String) , DarkTheme , }