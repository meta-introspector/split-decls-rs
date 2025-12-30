// Generated macro for impl_258 (impl)
macro_rules! Depcrate_hedgeimpl_258 {
() => {
// Module: crate::hedge
// Provides: {"impl_258"}
// Dependencies: {}
impl < S , Request > std :: future :: Future for Future < S , Request > where S : tower_service :: Service < Request > , S :: Error : Into < crate :: BoxError > , { type Output = Result < S :: Response , crate :: BoxError > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { self . project () . inner . poll (cx) . map_err (Into :: into) } }
};
}
