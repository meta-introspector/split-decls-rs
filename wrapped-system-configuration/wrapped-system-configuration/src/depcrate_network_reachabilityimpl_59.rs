// Generated macro for impl_59 (impl)
macro_rules! Depcrate_network_reachabilityimpl_59 {
() => {
// Module: crate::network_reachability
// Provides: {"impl_59"}
// Dependencies: {}
impl < T : Fn (ReachabilityFlags) + Sync + Send > NetworkReachabilityCallbackContext < T > { fn new (host : SCNetworkReachability , callback : T) -> Self { Self { _host : host , callback , } } extern "C" fn callback (_target : SCNetworkReachabilityRef , flags : SCNetworkReachabilityFlags , context : * mut c_void ,) { let context : & mut Self = unsafe { & mut (* (context as * mut _)) } ; (context . callback) (ReachabilityFlags :: from_bits_retain (flags)) ; } extern "C" fn copy_ctx_description (_ctx : * const c_void) -> CFStringRef { let description = CFString :: from_static_string ("NetworkRechability's callback context") ; let description_ref = description . as_concrete_TypeRef () ; std :: mem :: forget (description) ; description_ref } extern "C" fn release_context (ctx : * const c_void) { unsafe { Arc :: decrement_strong_count (ctx as * mut Self) ; } } extern "C" fn retain_context (ctx_ptr : * const c_void) -> * const c_void { unsafe { Arc :: increment_strong_count (ctx_ptr as * mut Self) ; } ctx_ptr } }
};
}
