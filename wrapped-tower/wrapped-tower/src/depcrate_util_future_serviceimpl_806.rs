// Generated macro for impl_806 (impl)
macro_rules! Depcrate_util_future_serviceimpl_806 {
() => {
// Module: crate::util::future_service
// Provides: {"impl_806"}
// Dependencies: {}
impl < F , S > fmt :: Debug for FutureService < F , S > where S : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("FutureService") . field ("state" , & format_args ! ("{:?}" , self . state)) . finish () } }
};
}
