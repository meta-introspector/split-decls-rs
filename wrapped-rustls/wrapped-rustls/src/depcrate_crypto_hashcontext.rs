// Generated macro for Context (trait)
macro_rules! Depcrate_crypto_hashContext {
() => {
// Module: crate::crypto::hash
// Provides: {"Context"}
// Dependencies: {}
# [doc = " How to incrementally compute a hash."] pub trait Context : Send + Sync { # [doc = " Finish the computation, returning the resulting output."] # [doc = ""] # [doc = " The computation remains valid, and more data can be added later with"] # [doc = " [`Context::update()`]."] # [doc = ""] # [doc = " Compare with [`Context::finish()`] which consumes the computation"] # [doc = " and prevents any further data being added.  This can be more efficient"] # [doc = " because it avoids a hash context copy to apply Merkle-Damgård padding"] # [doc = " (if required)."] fn fork_finish (& self) -> Output ; # [doc = " Fork the computation, producing another context that has the"] # [doc = " same prefix as this one."] fn fork (& self) -> Box < dyn Context > ; # [doc = " Terminate and finish the computation, returning the resulting output."] # [doc = ""] # [doc = " Further data cannot be added after this, because the context is consumed."] # [doc = " Compare [`Context::fork_finish()`]."] fn finish (self : Box < Self >) -> Output ; # [doc = " Add `data` to computation."] fn update (& mut self , data : & [u8]) ; }
};
}
