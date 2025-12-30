// Generated macro for impl_34 (impl)
macro_rules! Depcrate_to_valueimpl_34 {
() => {
// Module: crate::to_value
// Provides: {"impl_34"}
// Dependencies: {}
impl < V : serde_core :: Serialize > sval :: Value for ToValue < V > { fn stream < 'sval , S : sval :: Stream < 'sval > + ? Sized > (& 'sval self , stream : & mut S) -> sval :: Result { self . 0 . serialize (Stream { stream }) ? ; Ok (()) } }
};
}
