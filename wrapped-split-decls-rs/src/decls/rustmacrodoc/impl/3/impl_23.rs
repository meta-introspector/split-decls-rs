use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'ast > Visit < 'ast > for IncludeVisitor { fn visit_macro (& mut self , mac : & 'ast Macro) { if mac . path . is_ident ("include") { if let Ok (lit_str) = mac . parse_body :: < LitStr > () { let include_path = self . current_file_path . parent () . unwrap () . join (lit_str . value ()) ; self . includes . push (include_path) ; } } syn :: visit :: visit_macro (self , mac) ; } }