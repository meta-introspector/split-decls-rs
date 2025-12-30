// Generated macro for impl_46 (impl)
macro_rules! Depcrate_to_valueimpl_46 {
() => {
// Module: crate::to_value
// Provides: {"impl_46"}
// Dependencies: {}
impl < 'sval , S : sval :: Stream < 'sval > > Stream < S > { fn stream_value (& mut self , v : impl sval :: Value) -> Result < () , Error > { self . stream . value_computed (& v) ? ; Ok (()) } }
};
}
