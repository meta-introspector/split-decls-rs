// Generated macro for impl_144 (impl)
macro_rules! Depcrate_intrinsicimpl_144 {
() => {
// Module: crate::intrinsic
// Provides: {"impl_144"}
// Dependencies: {}
impl Signature { pub fn drop_argument (& mut self , arg_name : & WildString) -> Result < () , String > { if let Some (idx) = self . arguments . iter () . position (| arg | arg . name . to_string () == arg_name . to_string ()) { self . arguments . remove (idx) ; Ok (()) } else { Err (format ! ("no argument {arg_name} found to drop")) } } pub fn build (& mut self , ctx : & LocalContext) -> context :: Result { if self . name_has_neon_suffix () { self . name . build_neon_intrinsic_signature (ctx) ? ; } else { self . name . build_acle (ctx) ? ; } # [allow (clippy :: collapsible_if)] if let Some (ref mut return_type) = self . return_type { if let Some (w) = return_type . clone () . wildcard () { return_type . populate_wildcard (ctx . provide_type_wildcard (w) ?) ? ; } } self . arguments . iter_mut () . try_for_each (| arg | arg . name . build_acle (ctx)) ? ; self . arguments . iter_mut () . filter_map (| arg | { arg . kind . clone () . wildcard () . map (| w | (& mut arg . kind , w . clone ())) }) . try_for_each (| (ty , w) | ty . populate_wildcard (ctx . provide_type_wildcard (& w) ?)) } pub fn fn_name (& self) -> WildString { self . name . replace (['[' , ']'] , "") } pub fn doc_name (& self) -> String { self . name . to_string () } fn name_has_neon_suffix (& self) -> bool { for part in self . name . wildcards () { let has_suffix = match part { Wildcard :: NEONType (_ , _ , suffix_type) => suffix_type . is_some () , _ => false , } ; if has_suffix { return true ; } } false } }
};
}
