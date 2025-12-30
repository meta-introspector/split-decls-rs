// Generated macro for impl_35 (impl)
macro_rules! Depcrate_msgs_baseimpl_35 {
() => {
// Module: crate::msgs::base
// Provides: {"impl_35"}
// Dependencies: {}
impl < C : Cardinality > PayloadU24 < '_ , C > { pub (crate) fn into_owned (self) -> PayloadU24 < 'static , C > { PayloadU24 (self . 0 . into_owned () , PhantomData) } pub (crate) fn into_vec (self) -> Vec < u8 > { self . 0 . into_owned () . into_vec () } }
};
}
