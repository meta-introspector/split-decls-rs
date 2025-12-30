// Generated macro for impl_41 (impl)
macro_rules! Depcrate_msgs_baseimpl_41 {
() => {
// Module: crate::msgs::base
// Provides: {"impl_41"}
// Dependencies: {}
impl < C : Cardinality > PayloadU16 < C > { pub (crate) fn new (bytes : Vec < u8 >) -> Self { debug_assert ! (bytes . len () >= C :: MIN) ; Self (bytes , PhantomData) } }
};
}
