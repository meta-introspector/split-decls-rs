use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: lint_severity_at");
fn lint_severity_at (sema : & Semantics < '_ , RootDatabase > , node : & InFile < SyntaxNode > , lint_groups : & LintGroups , edition : Edition ,) -> Option < Severity > { node . value . ancestors () . filter_map (ast :: AnyHasAttrs :: cast) . find_map (| ancestor | { lint_attrs (sema , ancestor , edition) . find_map (| (lint , severity) | lint_groups . contains (& lint) . then_some (severity)) }) . or_else (| | { lint_severity_at (sema , & sema . find_parent_file (node . file_id) ? , lint_groups , edition ,) }) }
}