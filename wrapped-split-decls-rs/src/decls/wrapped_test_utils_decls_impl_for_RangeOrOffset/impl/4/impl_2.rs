use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl RangeOrOffset { pub fn expect_offset (self) -> TextSize { match self { RangeOrOffset :: Offset (it) => it , RangeOrOffset :: Range (_) => { panic ! ("expected an offset but got a range instead") } } } pub fn expect_range (self) -> TextRange { match self { RangeOrOffset :: Range (it) => it , RangeOrOffset :: Offset (_) => panic ! ("expected a range but got an offset") , } } pub fn range_or_empty (self) -> TextRange { match self { RangeOrOffset :: Range (range) => range , RangeOrOffset :: Offset (offset) => TextRange :: empty (offset) , } } }
}