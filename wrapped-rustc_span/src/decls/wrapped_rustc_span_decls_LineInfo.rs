use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct LineInfo {
    /// Index of line, starting from 0.
    pub line_index: usize,
    /// Column in line where span begins, starting from 0.
    pub start_col: CharPos,
    /// Column in line where span ends, starting from 0, exclusive.
    pub end_col: CharPos,
}
