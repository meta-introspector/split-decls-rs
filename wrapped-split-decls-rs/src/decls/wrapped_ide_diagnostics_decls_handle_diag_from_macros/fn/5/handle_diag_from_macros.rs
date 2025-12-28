use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Returns whether to keep this diagnostic (or remove it)."] fn handle_diag_from_macros (sema : & Semantics < '_ , RootDatabase > , diag : & mut Diagnostic , node : & InFile < SyntaxNode > ,) -> bool { let Some (macro_file) = node . file_id . macro_file () else { return true } ; let span_map = sema . db . expansion_span_map (macro_file) ; let mut spans = span_map . spans_for_range (node . text_range ()) ; if spans . any (| span | { span . ctx . outer_expn (sema . db) . is_some_and (| expansion | { let macro_call = sema . db . lookup_intern_macro_call (expansion . into ()) ; ! Crate :: from (macro_call . def . krate) . origin (sema . db) . is_local () || ! macro_call . def . kind . is_declarative () }) }) { diag . fixes = None ; if let DiagnosticCode :: RustcLint (lint) = diag . code && ! LINTS_TO_REPORT_IN_EXTERNAL_MACROS . contains (lint) { return false ; } } true }
}