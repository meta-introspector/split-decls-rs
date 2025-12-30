// Generated macro for impl_222 (impl)
macro_rules! Depcrate_ich_hcximpl_222 {
() => {
// Module: crate::ich::hcx
// Provides: {"impl_222"}
// Dependencies: {}
impl < 'a > StableHashingContext < 'a > { # [inline] pub fn new (sess : & 'a Session , untracked : & 'a Untracked) -> Self { let hash_spans_initial = ! sess . opts . unstable_opts . incremental_ignore_spans ; StableHashingContext { untracked , incremental_ignore_spans : sess . opts . unstable_opts . incremental_ignore_spans , caching_source_map : None , raw_source_map : sess . source_map () , hashing_controls : HashingControls { hash_spans : hash_spans_initial } , } } # [inline] pub fn while_hashing_spans < F : FnOnce (& mut Self) > (& mut self , hash_spans : bool , f : F) { let prev_hash_spans = self . hashing_controls . hash_spans ; self . hashing_controls . hash_spans = hash_spans ; f (self) ; self . hashing_controls . hash_spans = prev_hash_spans ; } # [inline] pub fn def_path_hash (& self , def_id : DefId) -> DefPathHash { if let Some (def_id) = def_id . as_local () { self . local_def_path_hash (def_id) } else { self . untracked . cstore . read () . def_path_hash (def_id) } } # [inline] pub fn local_def_path_hash (& self , def_id : LocalDefId) -> DefPathHash { self . untracked . definitions . read () . def_path_hash (def_id) } # [inline] pub fn source_map (& mut self) -> & mut CachingSourceMapView < 'a > { match self . caching_source_map { Some (ref mut sm) => sm , ref mut none => { * none = Some (CachingSourceMapView :: new (self . raw_source_map)) ; none . as_mut () . unwrap () } } } # [inline] pub fn is_ignored_attr (& self , name : Symbol) -> bool { ich :: IGNORED_ATTRIBUTES . contains (& name) } # [inline] pub fn hashing_controls (& self) -> HashingControls { self . hashing_controls . clone () } }
};
}
