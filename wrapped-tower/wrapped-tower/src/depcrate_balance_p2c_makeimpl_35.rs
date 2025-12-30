// Generated macro for impl_35 (impl)
macro_rules! Depcrate_balance_p2c_makeimpl_35 {
() => {
// Module: crate::balance::p2c::make
// Provides: {"impl_35"}
// Dependencies: {}
impl < F , Req > fmt :: Debug for MakeFuture < F , Req > where F : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let Self { inner , _marker } = self ; f . debug_struct ("MakeFuture") . field ("inner" , inner) . finish () } }
};
}
