// Generated macro for impl_423 (impl)
macro_rules! Depcrate_compression_utilsimpl_423 {
() => {
// Module: crate::compression_utils
// Provides: {"impl_423"}
// Dependencies: {}
impl < S , E > StreamErrorIntoIoError < S , E > { pub (crate) fn new (inner : S) -> Self { Self { inner , error : None } } # [doc = " Get a reference to the inner body"] pub (crate) fn get_ref (& self) -> & S { & self . inner } # [doc = " Get a mutable reference to the inner inner"] pub (crate) fn get_mut (& mut self) -> & mut S { & mut self . inner } # [doc = " Get a pinned mutable reference to the inner inner"] pub (crate) fn get_pin_mut (self : Pin < & mut Self >) -> Pin < & mut S > { self . project () . inner } # [doc = " Consume `self`, returning the inner inner"] pub (crate) fn into_inner (self) -> S { self . inner } }
};
}
