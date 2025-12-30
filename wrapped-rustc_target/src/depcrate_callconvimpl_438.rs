// Generated macro for impl_438 (impl)
macro_rules! Depcrate_callconvimpl_438 {
() => {
// Module: crate::callconv
// Provides: {"impl_438"}
// Dependencies: {}
impl < 'a , Ty : fmt :: Display > fmt :: Debug for ArgAbi < 'a , Ty > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let ArgAbi { layout , mode } = self ; f . debug_struct ("ArgAbi") . field ("layout" , layout) . field ("mode" , mode) . finish () } }
};
}
