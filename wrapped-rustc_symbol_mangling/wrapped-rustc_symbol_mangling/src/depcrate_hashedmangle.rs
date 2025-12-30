// Generated macro for mangle (function)
macro_rules! Depcrate_hashedmangle {
() => {
// Module: crate::hashed
// Provides: {"mangle"}
// Dependencies: {}
pub (super) fn mangle < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > , instantiating_crate : Option < CrateNum > , full_mangling_name : impl FnOnce () -> String ,) -> String { let crate_num = if let Some (krate) = instantiating_crate { krate } else { instance . def_id () . krate } ; let mut symbol = "_RNxC" . to_string () ; v0 :: push_ident (tcx . crate_name (crate_num) . as_str () , & mut symbol) ; let hash = tcx . with_stable_hashing_context (| mut hcx | { let mut hasher = StableHasher :: new () ; full_mangling_name () . hash_stable (& mut hcx , & mut hasher) ; hasher . finish :: < Hash64 > () . as_u64 () }) ; push_hash64 (hash , & mut symbol) ; symbol }
};
}
