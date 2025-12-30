// Generated macro for impl_194 (impl)
macro_rules! Depcrate_filter_unifyimpl_194 {
() => {
// Module: crate::filter::unify
// Provides: {"impl_194"}
// Dependencies: {}
impl < F , T > Future for UnifyFuture < F > where F : TryFuture < Ok = (Either < T , T > ,) > , { type Output = Result < T , F :: Error > ; # [inline] fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { Poll :: Ready (match ready ! (self . project () . inner . try_poll (cx)) ? { (Either :: A (x) ,) | (Either :: B (x) ,) => Ok (x) , }) } }
};
}
