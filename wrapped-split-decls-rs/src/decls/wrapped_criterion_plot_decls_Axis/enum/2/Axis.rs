use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A coordinate axis"] # [derive (Clone , Copy)] pub enum Axis { # [doc = " X axis on the bottom side of the figure"] BottomX , # [doc = " Y axis on the left side of the figure"] LeftY , # [doc = " Y axis on the right side of the figure"] RightY , # [doc = " X axis on the top side of the figure"] TopX , }