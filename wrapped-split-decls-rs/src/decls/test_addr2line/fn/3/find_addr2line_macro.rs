use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Find a specific addr2line macro by partial name match"] fn find_addr2line_macro (macros : & [(String , split_decls_rs :: output2_macro_system :: MacroDeclaration)] , target : & str) -> Option < (String , split_decls_rs :: output2_macro_system :: MacroDeclaration) > { macros . iter () . find (| (name , _) | name . contains (target)) . map (| (name , decl) | (name . clone () , decl . clone ())) }