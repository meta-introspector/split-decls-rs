use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: escape_html");
# [doc = " Escape tags in such a way that it is suitable for inclusion in a"] # [doc = " Graphviz HTML label."] pub fn escape_html (s : & str) -> String { s . replace ('&' , "&amp;") . replace ('\"' , "&quot;") . replace ('<' , "&lt;") . replace ('>' , "&gt;") . replace ('\n' , "<br align=\"left\"/>") }
}