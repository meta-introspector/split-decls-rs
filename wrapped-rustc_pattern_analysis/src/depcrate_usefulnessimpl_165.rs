// Generated macro for impl_165 (impl)
macro_rules! Depcrate_usefulnessimpl_165 {
() => {
// Module: crate::usefulness
// Provides: {"impl_165"}
// Dependencies: {}
impl < 'a , Cx : PatCx > fmt :: Debug for PlaceCtxt < 'a , Cx > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("PlaceCtxt") . field ("ty" , self . ty) . finish () } }
};
}
