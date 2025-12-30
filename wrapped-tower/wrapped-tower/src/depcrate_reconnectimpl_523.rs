// Generated macro for impl_523 (impl)
macro_rules! Depcrate_reconnectimpl_523 {
() => {
// Module: crate::reconnect
// Provides: {"impl_523"}
// Dependencies: {}
impl < M , Target > fmt :: Debug for Reconnect < M , Target > where M : Service < Target > + fmt :: Debug , M :: Future : fmt :: Debug , M :: Response : fmt :: Debug , Target : fmt :: Debug , { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { fmt . debug_struct ("Reconnect") . field ("mk_service" , & self . mk_service) . field ("state" , & self . state) . field ("target" , & self . target) . finish () } }
};
}
