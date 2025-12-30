use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: find_outline_mod_lint_severity");
fn find_outline_mod_lint_severity (sema : & Semantics < '_ , RootDatabase > , node : & InFile < SyntaxNode > , diag : & Diagnostic , edition : Edition ,) -> Option < Severity > { let mod_node = node . value . ancestors () . find_map (ast :: Module :: cast) ? ; if mod_node . item_list () . is_some () { return None ; } let mod_def = sema . to_module_def (& mod_node) ? ; let module_source_file = sema . module_definition_node (mod_def) ; let mut result = None ; let lint_groups = lint_groups (& diag . code , edition) ; lint_attrs (sema , ast :: AnyHasAttrs :: cast (module_source_file . value) . expect ("SourceFile always has attrs") , edition ,) . for_each (| (lint , severity) | { if lint_groups . contains (& lint) { result = Some (severity) ; } }) ; result }
}