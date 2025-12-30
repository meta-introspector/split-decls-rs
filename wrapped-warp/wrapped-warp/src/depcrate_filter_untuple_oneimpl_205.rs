// Generated macro for impl_205 (impl)
macro_rules! Depcrate_filter_untuple_oneimpl_205 {
() => {
// Module: crate::filter::untuple_one
// Provides: {"impl_205"}
// Dependencies: {}
impl < F , T > Future for UntupleOneFuture < F > where F : Filter < Extract = (T ,) > , T : Tuple , { type Output = Result < T , F :: Error > ; # [inline] fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { match ready ! (self . project () . extract . try_poll (cx)) { Ok ((t ,)) => Poll :: Ready (Ok (t)) , Err (err) => Poll :: Ready (Err (err)) , } } }
};
}
