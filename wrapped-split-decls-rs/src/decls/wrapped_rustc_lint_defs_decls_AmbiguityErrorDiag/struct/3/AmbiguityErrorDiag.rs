use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug)] pub struct AmbiguityErrorDiag { pub msg : String , pub span : Span , pub label_span : Span , pub label_msg : String , pub note_msg : String , pub b1_span : Span , pub b1_note_msg : String , pub b1_help_msgs : Vec < String > , pub b2_span : Span , pub b2_note_msg : String , pub b2_help_msgs : Vec < String > , }
}