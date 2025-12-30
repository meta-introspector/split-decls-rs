// Generated macro for impl_85 (impl)
macro_rules! Depcrate_msgs_codecimpl_85 {
() => {
// Module: crate::msgs::codec
// Provides: {"impl_85"}
// Dependencies: {}
impl Codec < '_ > for () { fn encode (& self , _ : & mut Vec < u8 >) { } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { r . expect_empty ("Empty") } }
};
}
