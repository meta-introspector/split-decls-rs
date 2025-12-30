// Generated macro for impl_994 (impl)
macro_rules! Depcrate_timeout_serviceimpl_994 {
() => {
// Module: crate::timeout::service
// Provides: {"impl_994"}
// Dependencies: {}
impl < Fut , ResBody , E > Future for ResponseBodyTimeoutFuture < Fut > where Fut : Future < Output = Result < Response < ResBody > , E > > , { type Output = Result < Response < TimeoutBody < ResBody > > , E > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let timeout = self . timeout ; let this = self . project () ; let res = ready ! (this . inner . poll (cx)) ? ; Poll :: Ready (Ok (res . map (| body | TimeoutBody :: new (timeout , body)))) } }
};
}
