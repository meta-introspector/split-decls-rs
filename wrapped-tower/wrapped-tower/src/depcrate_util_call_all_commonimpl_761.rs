// Generated macro for impl_761 (impl)
macro_rules! Depcrate_util_call_all_commonimpl_761 {
() => {
// Module: crate::util::call_all::common
// Provides: {"impl_761"}
// Dependencies: {}
impl < Svc , S , Q > fmt :: Debug for CallAll < Svc , S , Q > where Svc : fmt :: Debug , S : Stream + fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("CallAll") . field ("service" , & self . service) . field ("stream" , & self . stream) . field ("eof" , & self . eof) . finish () } }
};
}
