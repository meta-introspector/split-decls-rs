// Generated macro for impl_34 (impl)
macro_rules! Depcrate_balance_p2c_makeimpl_34 {
() => {
// Module: crate::balance::p2c::make
// Provides: {"impl_34"}
// Dependencies: {}
impl < F , T , E , Req > Future for MakeFuture < F , Req > where F : Future < Output = Result < T , E > > , T : Discover , < T as Discover > :: Key : Hash , < T as Discover > :: Service : Service < Req > , < < T as Discover > :: Service as Service < Req > > :: Error : Into < crate :: BoxError > , { type Output = Result < Balance < T , Req > , E > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; let inner = ready ! (this . inner . poll (cx)) ? ; let svc = Balance :: new (inner) ; Poll :: Ready (Ok (svc)) } }
};
}
