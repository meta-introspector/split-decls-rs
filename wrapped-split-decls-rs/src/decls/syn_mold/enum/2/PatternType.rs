use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone)] pub enum PatternType { ParseQuote , VisitMut , TokenStreamGeneration , AttributeProcessing , MacroExpansion , }