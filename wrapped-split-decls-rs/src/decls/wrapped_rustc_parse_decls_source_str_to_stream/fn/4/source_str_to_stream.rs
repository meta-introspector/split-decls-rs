use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: source_str_to_stream");
# [doc = " Given a source string, produces a sequence of token trees."] # [doc = ""] # [doc = " NOTE: This only strips shebangs, not frontmatter!"] pub fn source_str_to_stream (psess : & ParseSess , name : FileName , source : String , override_span : Option < Span > ,) -> Result < TokenStream , Vec < Diag < '_ > > > { let source_file = psess . source_map () . new_source_file (name , source) ; source_file_to_stream (psess , source_file , override_span , StripTokens :: Shebang) }
}