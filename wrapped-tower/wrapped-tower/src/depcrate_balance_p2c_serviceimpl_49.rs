// Generated macro for impl_49 (impl)
macro_rules! Depcrate_balance_p2c_serviceimpl_49 {
() => {
// Module: crate::balance::p2c::service
// Provides: {"impl_49"}
// Dependencies: {}
impl < D : Discover , Req > fmt :: Debug for Balance < D , Req > where D : fmt :: Debug , D :: Key : Hash + fmt :: Debug , D :: Service : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Balance") . field ("discover" , & self . discover) . field ("services" , & self . services) . finish () } }
};
}
