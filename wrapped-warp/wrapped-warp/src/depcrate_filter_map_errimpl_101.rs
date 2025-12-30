// Generated macro for impl_101 (impl)
macro_rules! Depcrate_filter_map_errimpl_101 {
() => {
// Module: crate::filter::map_err
// Provides: {"impl_101"}
// Dependencies: {}
impl < T , F , E > Future for MapErrFuture < T , F > where T : Filter , F : Fn (T :: Error) -> E , { type Output = Result < T :: Extract , E > ; # [inline] fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { self . as_mut () . project () . extract . try_poll (cx) . map_err (| err | (self . callback) (err)) } }
};
}
