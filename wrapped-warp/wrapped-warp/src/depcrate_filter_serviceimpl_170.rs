// Generated macro for impl_170 (impl)
macro_rules! Depcrate_filter_serviceimpl_170 {
() => {
// Module: crate::filter::service
// Provides: {"impl_170"}
// Dependencies: {}
impl < F > Future for FilteredFuture < F > where F : TryFuture , F :: Ok : Reply , F :: Error : IsReject , { type Output = Result < Response , Infallible > ; # [inline] fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { debug_assert ! (! route :: is_set () , "nested route::set calls") ; let pin = self . project () ; let fut = pin . future ; match route :: set (pin . route , | | fut . try_poll (cx)) { Poll :: Ready (Ok (ok)) => Poll :: Ready (Ok (ok . into_response ())) , Poll :: Pending => Poll :: Pending , Poll :: Ready (Err (err)) => { tracing :: debug ! ("rejected: {:?}" , err) ; Poll :: Ready (Ok (err . into_response ())) } } } }
};
}
