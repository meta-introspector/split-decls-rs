use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type FileLinesResult = Result < FileLines , SpanLinesError > ;