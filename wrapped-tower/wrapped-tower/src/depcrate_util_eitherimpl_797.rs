// Generated macro for impl_797 (impl)
macro_rules! Depcrate_util_eitherimpl_797 {
() => {
// Module: crate::util::either
// Provides: {"impl_797"}
// Dependencies: {}
impl < A , B > Future for EitherResponseFuture < A , B > where A : Future , B : Future < Output = A :: Output > , { type Output = A :: Output ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { match self . project () . kind . project () { KindProj :: Left { inner } => inner . poll (cx) , KindProj :: Right { inner } => inner . poll (cx) , } } }
};
}
