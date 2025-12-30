// Generated macro for impl_752 (impl)
macro_rules! Depcrate_connimpl_752 {
() => {
// Module: crate::conn
// Provides: {"impl_752"}
// Dependencies: {}
impl < 'a , Side : SideData > From < & 'a mut ConnectionCommon < Side > > for Context < 'a , Side > { fn from (conn : & 'a mut ConnectionCommon < Side >) -> Self { Self { common : & mut conn . core . common_state , data : & mut conn . core . side , sendable_plaintext : Some (& mut conn . sendable_plaintext) , } } }
};
}
