use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug)] struct DuplicateScanner { macros : HashMap < String , MacroDeclaration > , patterns : HashMap < String , CodePattern > , }