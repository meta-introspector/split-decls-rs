use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Debug , Clone)] pub enum PatternType { ParseQuote , VisitMut , TokenStreamGeneration , AttributeProcessing , MacroExpansion , }
}