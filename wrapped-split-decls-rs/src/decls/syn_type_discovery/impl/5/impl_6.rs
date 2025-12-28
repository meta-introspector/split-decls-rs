use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'ast > Visit < 'ast > for TypeExtractorVisitor < '_ > { fn visit_item_enum (& mut self , node : & 'ast syn :: ItemEnum) { let enum_name = node . ident . to_string () ; if self . is_syn_ast_enum (& enum_name) { self . discovery . discovered_types . insert (enum_name . clone ()) ; self . discovery . visit_methods . insert (format ! ("visit_{}" , enum_name . to_lowercase ())) ; for variant in & node . variants { let variant_name = variant . ident . to_string () ; self . discovery . enum_variants . insert (variant_name . clone ()) ; self . discovery . discovered_types . insert (variant_name . clone ()) ; self . discovery . visit_methods . insert (format ! ("visit_{}" , variant_name . to_lowercase ())) ; } } syn :: visit :: visit_item_enum (self , node) ; } }
}