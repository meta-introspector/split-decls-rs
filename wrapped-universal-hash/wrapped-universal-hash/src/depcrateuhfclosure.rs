// Generated macro for UhfClosure (trait)
macro_rules! DepcrateUhfClosure {
() => {
// Module: crate
// Provides: {"UhfClosure"}
// Dependencies: {}
# [doc = " Trait for [`UhfBackend`] users."] # [doc = ""] # [doc = " This trait is used to define rank-2 closures."] pub trait UhfClosure : BlockSizeUser { # [doc = " Execute closure with the provided UHF backend."] fn call < B : UhfBackend < BlockSize = Self :: BlockSize > > (self , backend : & mut B) ; }
};
}
