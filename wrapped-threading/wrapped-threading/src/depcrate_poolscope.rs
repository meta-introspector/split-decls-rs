// Generated macro for Scope (struct)
macro_rules! Depcrate_poolScope {
() => {
// Module: crate::pool
// Provides: {"Scope"}
// Dependencies: {}
# [doc = " A scope to submit closures in."] # [doc = ""] # [doc = " See [`scope`][Pool::scope] for details."] pub struct Scope < 'scope , 'env : 'scope > { pool : & 'scope Pool , scope : PhantomData < & 'scope mut & 'scope () > , env : PhantomData < & 'env mut & 'env () > , }
};
}
