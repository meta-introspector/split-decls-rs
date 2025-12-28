use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Clone)] pub struct DiagnosticsConfig { # [doc = " Whether native diagnostics are enabled."] pub enabled : bool , pub proc_macros_enabled : bool , pub proc_attr_macros_enabled : bool , pub disable_experimental : bool , pub disabled : FxHashSet < String > , pub expr_fill_default : ExprFillDefaultMode , pub style_lints : bool , pub snippet_cap : Option < SnippetCap > , pub insert_use : InsertUseConfig , pub prefer_no_std : bool , pub prefer_prelude : bool , pub prefer_absolute : bool , pub term_search_fuel : u64 , pub term_search_borrowck : bool , }
}