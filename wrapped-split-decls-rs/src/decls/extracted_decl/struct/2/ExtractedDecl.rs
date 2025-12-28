use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Represents a single extracted declaration."] # [derive (Debug , Clone)] pub struct ExtractedDecl { pub name : String , pub kind : String , pub content : TokenStream , pub metadata : ExtractedDeclMetadata , pub source_map : std :: collections :: HashMap < usize , SourceLocation > , }
}