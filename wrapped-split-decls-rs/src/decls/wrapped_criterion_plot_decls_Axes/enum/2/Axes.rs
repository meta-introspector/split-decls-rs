use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A pair of axes that define a coordinate system"] # [allow (missing_docs)] # [derive (Clone , Copy)] pub enum Axes { BottomXLeftY , BottomXRightY , TopXLeftY , TopXRightY , }