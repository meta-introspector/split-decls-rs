// Generated macro for Scope (struct)
macro_rules! Depcrate_thread_scopeScope {
() => {
// Module: crate::thread::scope
// Provides: {"Scope"}
// Dependencies: {}
# [doc = " See [`std::thread::Scope`]."] # [derive (Debug)] pub struct Scope < 'scope , 'env : 'scope > { # [doc = " Implementation of [`Scope`]."] pub (super) this : r#impl :: Scope , # [doc = " Invariance over the lifetime `'scope`."] # [allow (clippy :: struct_field_names)] pub (super) _scope : PhantomData < & 'scope mut & 'scope () > , # [doc = " Invariance over the lifetime `'env`."] pub (super) _env : PhantomData < & 'env mut & 'env () > , }
};
}
