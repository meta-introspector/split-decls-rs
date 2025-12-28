use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl TypeExtractorVisitor < '_ > { fn is_syn_ast_enum (& self , name : & str) -> bool { matches ! (name , "Item" | "Expr" | "Type" | "Pat" | "Stmt" | "Lit") || name . starts_with ("Item") || name . starts_with ("Expr") || name . starts_with ("Type") || name . starts_with ("Pat") || name . starts_with ("Lit") } }