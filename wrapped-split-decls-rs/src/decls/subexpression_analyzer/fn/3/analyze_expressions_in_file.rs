use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn analyze_expressions_in_file (file_path : & Path) -> Result < HashMap < String , Vec < String > > > { let content = fs :: read_to_string (file_path) ? ; let syntax_tree : File = syn :: parse_file (& content) ? ; let mut visitor = ExpressionVisitor :: new (file_path . to_string_lossy () . to_string ()) ; for item in & syntax_tree . items { syn :: visit :: visit_item (& mut visitor , item) ; } Ok (visitor . expressions) }