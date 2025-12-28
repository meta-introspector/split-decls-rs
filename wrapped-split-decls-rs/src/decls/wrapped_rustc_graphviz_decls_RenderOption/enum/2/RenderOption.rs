use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Clone , PartialEq , Eq , Debug)] pub enum RenderOption { NoEdgeLabels , NoNodeLabels , NoEdgeStyles , NoNodeStyles , Fontname (String) , DarkTheme , }
}