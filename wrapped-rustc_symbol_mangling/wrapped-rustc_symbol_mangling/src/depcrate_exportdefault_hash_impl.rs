// Generated macro for default_hash_impl (macro)
macro_rules! Depcrate_exportdefault_hash_impl {
() => {
// Module: crate::export
// Provides: {"default_hash_impl"}
// Dependencies: {}
macro_rules ! default_hash_impl { ($ ($ t : ty ,) +) => { $ (impl <'tcx > AbiHashStable <'tcx > for $ t { # [inline] fn abi_hash (& self , _tcx : TyCtxt <'tcx >, hasher : & mut StableHasher) { :: std :: hash :: Hash :: hash (self , hasher) ; } }) * } ; }
};
}
