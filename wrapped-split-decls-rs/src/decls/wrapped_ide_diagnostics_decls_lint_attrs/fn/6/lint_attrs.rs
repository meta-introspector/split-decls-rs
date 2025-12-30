use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: lint_attrs");
fn lint_attrs < 'a > (sema : & 'a Semantics < 'a , RootDatabase > , ancestor : ast :: AnyHasAttrs , edition : Edition ,) -> impl Iterator < Item = (SmolStr , Severity) > + 'a { ancestor . attrs_including_inner () . filter_map (| attr | { attr . as_simple_call () . and_then (| (name , value) | match & * name { "allow" | "expect" => Some (Either :: Left (iter :: once ((Severity :: Allow , value)))) , "warn" => Some (Either :: Left (iter :: once ((Severity :: Warning , value)))) , "forbid" | "deny" => Some (Either :: Left (iter :: once ((Severity :: Error , value)))) , "cfg_attr" => { let mut lint_attrs = Vec :: new () ; cfg_attr_lint_attrs (sema , & value , & mut lint_attrs) ; Some (Either :: Right (lint_attrs . into_iter ())) } _ => None , }) }) . flatten () . flat_map (move | (severity , lints) | { parse_tt_as_comma_sep_paths (lints , edition) . into_iter () . flat_map (move | lints | { lints . into_iter () . map (move | lint | { (lint . segments () . filter_map (| segment | segment . name_ref ()) . join ("::") . into () , severity ,) }) }) }) }
}