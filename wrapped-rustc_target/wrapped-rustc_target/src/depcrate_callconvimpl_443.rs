// Generated macro for impl_443 (impl)
macro_rules! Depcrate_callconvimpl_443 {
() => {
// Module: crate::callconv
// Provides: {"impl_443"}
// Dependencies: {}
impl < 'a , Ty : fmt :: Display > fmt :: Debug for FnAbi < 'a , Ty > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let FnAbi { args , ret , c_variadic , fixed_count , conv , can_unwind } = self ; f . debug_struct ("FnAbi") . field ("args" , args) . field ("ret" , ret) . field ("c_variadic" , c_variadic) . field ("fixed_count" , fixed_count) . field ("conv" , conv) . field ("can_unwind" , can_unwind) . finish () } }
};
}
