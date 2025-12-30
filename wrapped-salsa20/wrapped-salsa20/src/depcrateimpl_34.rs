// Generated macro for impl_34 (impl)
macro_rules! Depcrateimpl_34 {
() => {
// Module: crate
// Provides: {"impl_34"}
// Dependencies: {}
impl < R : Unsigned > SalsaCore < R > { # [doc = " Create new Salsa core from raw state."] # [doc = ""] # [doc = " This method is mainly intended for the `scrypt` crate."] # [doc = " Other users generally should not use this method."] pub fn from_raw_state (state : [u32 ; STATE_WORDS]) -> Self { Self { state , rounds : PhantomData , } } }
};
}
