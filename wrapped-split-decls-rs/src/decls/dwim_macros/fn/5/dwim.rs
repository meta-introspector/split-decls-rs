use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " DWIM (Do What I Mean) macro - deterministic macro discovery and generation"] # [doc = " "] # [doc = " Searches existing macros, finds best match, fails if ambiguous"] # [doc = " Iteratively generates macro calls, shares state via embedded ontology"] # [proc_macro] pub fn dwim (input : TokenStream) -> TokenStream { let intent = parse_macro_input ! (input as DwimIntent) ; let available_macros = discover_available_macros () ; match find_best_macro_match (& intent , & available_macros) { MacroMatch :: Single (macro_def) => { generate_macro_call (& macro_def , & intent) } , MacroMatch :: Ambiguous (matches) => { panic ! ("Ambiguous macro match: found {} candidates: {:?}" , matches . len () , matches) ; } , MacroMatch :: None => { iteratively_generate_macro (& intent) } } }
}