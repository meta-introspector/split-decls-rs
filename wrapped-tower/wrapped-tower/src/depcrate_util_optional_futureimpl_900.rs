// Generated macro for impl_900 (impl)
macro_rules! Depcrate_util_optional_futureimpl_900 {
() => {
// Module: crate::util::optional::future
// Provides: {"impl_900"}
// Dependencies: {}
impl < F , T , E > Future for ResponseFuture < F > where F : Future < Output = Result < T , E > > , E : Into < crate :: BoxError > , { type Output = Result < T , crate :: BoxError > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { match self . project () . inner . as_pin_mut () { Some (inner) => Poll :: Ready (Ok (ready ! (inner . poll (cx)) . map_err (Into :: into) ?)) , None => Poll :: Ready (Err (error :: None :: new () . into ())) , } } }
};
}
