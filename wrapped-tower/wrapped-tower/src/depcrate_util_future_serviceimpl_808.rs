// Generated macro for impl_808 (impl)
macro_rules! Depcrate_util_future_serviceimpl_808 {
() => {
// Module: crate::util::future_service
// Provides: {"impl_808"}
// Dependencies: {}
impl < F , S > fmt :: Debug for State < F , S > where S : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { State :: Future (_) => f . debug_tuple ("State::Future") . field (& format_args ! ("<{}>" , std :: any :: type_name ::< F > ())) . finish () , State :: Service (svc) => f . debug_tuple ("State::Service") . field (svc) . finish () , } } }
};
}
