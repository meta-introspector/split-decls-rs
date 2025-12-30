// Generated macro for impl_32 (impl)
macro_rules! Depcrate_balance_p2c_makeimpl_32 {
() => {
// Module: crate::balance::p2c::make
// Provides: {"impl_32"}
// Dependencies: {}
impl < S , Target , Req > Service < Target > for MakeBalance < S , Req > where S : Service < Target > , S :: Response : Discover , < S :: Response as Discover > :: Key : Hash , < S :: Response as Discover > :: Service : Service < Req > , < < S :: Response as Discover > :: Service as Service < Req > > :: Error : Into < crate :: BoxError > , { type Response = Balance < S :: Response , Req > ; type Error = S :: Error ; type Future = MakeFuture < S :: Future , Req > ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . inner . poll_ready (cx) } fn call (& mut self , target : Target) -> Self :: Future { MakeFuture { inner : self . inner . call (target) , _marker : PhantomData , } } }
};
}
