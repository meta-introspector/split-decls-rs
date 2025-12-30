// Generated macro for always_storage_live_locals (function)
macro_rules! Depcrate_impls_storage_livenessalways_storage_live_locals {
() => {
// Module: crate::impls::storage_liveness
// Provides: {"always_storage_live_locals"}
// Dependencies: {}
# [doc = " The set of locals in a MIR body that do not have `StorageLive`/`StorageDead` annotations."] # [doc = ""] # [doc = " These locals have fixed storage for the duration of the body."] pub fn always_storage_live_locals (body : & Body < '_ >) -> DenseBitSet < Local > { let mut always_live_locals = DenseBitSet :: new_filled (body . local_decls . len ()) ; for block in & * body . basic_blocks { for statement in & block . statements { if let StatementKind :: StorageLive (l) | StatementKind :: StorageDead (l) = statement . kind { always_live_locals . remove (l) ; } } } always_live_locals }
};
}
