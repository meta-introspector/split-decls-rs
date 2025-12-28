use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'ast > Visit < 'ast > for SynUsageVisitor { fn visit_item_fn (& mut self , node : & 'ast syn :: ItemFn) { let fn_name = node . sig . ident . to_string () ; self . analyze_block_for_syn_usage (& fn_name , & node . block) ; syn :: visit :: visit_item_fn (self , node) ; } fn visit_macro (& mut self , node : & 'ast syn :: Macro) { let macro_path = node . path . segments . iter () . map (| s | s . ident . to_string ()) . collect :: < Vec < _ > > () . join ("::") ; if macro_path . contains ("syn") || macro_path . contains ("quote") { self . patterns . entry (macro_path . clone ()) . or_insert_with (Vec :: new) . push ("macro_usage" . to_string ()) ; } syn :: visit :: visit_macro (self , node) ; } }
}