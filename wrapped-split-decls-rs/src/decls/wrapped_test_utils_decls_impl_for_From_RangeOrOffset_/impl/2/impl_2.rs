use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl From < RangeOrOffset > for TextRange { fn from (selection : RangeOrOffset) -> Self { match selection { RangeOrOffset :: Range (it) => it , RangeOrOffset :: Offset (it) => TextRange :: empty (it) , } } }