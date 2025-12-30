// Generated macro for impl_46 (impl)
macro_rules! Depcrate_msgs_baseimpl_46 {
() => {
// Module: crate::msgs::base
// Provides: {"impl_46"}
// Dependencies: {}
impl < C : Cardinality > PayloadU8 < C > { pub (crate) fn encode_slice (slice : & [u8] , bytes : & mut Vec < u8 >) { (slice . len () as u8) . encode (bytes) ; bytes . extend_from_slice (slice) ; } pub (crate) fn new (bytes : Vec < u8 >) -> Self { debug_assert ! (bytes . len () >= C :: MIN) ; Self (bytes , PhantomData) } }
};
}
