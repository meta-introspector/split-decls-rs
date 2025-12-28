use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl RealRustAstParser { fn extract_declarations_from_syn_file (syn_file : & syn :: File , file_path : & Path ,) -> Vec < Declaration > { let mut visitor = AstDeclarationVisitor :: new (file_path . to_path_buf ()) ; visitor . visit_file (syn_file) ; visitor . declarations } }
}