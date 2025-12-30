// Generated macro for mangle_internal_symbol (function)
macro_rules! Depcrate_v0mangle_internal_symbol {
() => {
// Module: crate::v0
// Provides: {"mangle_internal_symbol"}
// Dependencies: {}
pub fn mangle_internal_symbol < 'tcx > (tcx : TyCtxt < 'tcx > , item_name : & str) -> String { match item_name { "rust_eh_personality" => return item_name . to_owned () , "__isPlatformVersionAtLeast" | "__isOSVersionAtLeast" => return item_name . to_owned () , _ => { } } let prefix = "_R" ; let mut p : V0SymbolMangler < '_ > = V0SymbolMangler { tcx , start_offset : prefix . len () , is_exportable : false , paths : FxHashMap :: default () , types : FxHashMap :: default () , consts : FxHashMap :: default () , binders : vec ! [] , out : String :: from (prefix) , } ; p . path_append_ns (| p | { p . push ("C") ; p . push_disambiguator ({ let mut hasher = StableHasher :: new () ; hasher . write (tcx . sess . cfg_version . as_bytes ()) ; let hash : Hash64 = hasher . finish () ; hash . as_u64 () }) ; p . push_ident ("__rustc") ; Ok (()) } , 'v' , 0 , item_name ,) . unwrap () ; std :: mem :: take (& mut p . out) }
};
}
