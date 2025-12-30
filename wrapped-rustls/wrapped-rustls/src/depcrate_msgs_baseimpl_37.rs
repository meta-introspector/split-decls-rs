// Generated macro for impl_37 (impl)
macro_rules! Depcrate_msgs_baseimpl_37 {
() => {
// Module: crate::msgs::base
// Provides: {"impl_37"}
// Dependencies: {}
impl < 'a , C : Cardinality > From < Payload < 'a > > for PayloadU24 < 'a , C > { fn from (value : Payload < 'a >) -> Self { debug_assert ! (value . bytes () . len () >= C :: MIN) ; Self (value , PhantomData) } }
};
}
