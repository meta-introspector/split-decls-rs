// Generated macro for borrowed_locals (function)
macro_rules! Depcrate_impls_borrowed_localsborrowed_locals {
() => {
// Module: crate::impls::borrowed_locals
// Provides: {"borrowed_locals"}
// Dependencies: {}
# [doc = " The set of locals that are borrowed at some point in the MIR body."] pub fn borrowed_locals (body : & Body < '_ >) -> DenseBitSet < Local > { struct Borrowed (DenseBitSet < Local >) ; impl GenKill < Local > for Borrowed { # [inline] fn gen_ (& mut self , elem : Local) { self . 0 . gen_ (elem) } # [inline] fn kill (& mut self , _ : Local) { } } let mut borrowed = Borrowed (DenseBitSet :: new_empty (body . local_decls . len ())) ; TransferFunction { trans : & mut borrowed } . visit_body (body) ; borrowed . 0 }
};
}
