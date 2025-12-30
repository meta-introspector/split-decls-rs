// Generated macro for impl_418 (impl)
macro_rules! Depcrate_load_shed_futureimpl_418 {
() => {
// Module: crate::load_shed::future
// Provides: {"impl_418"}
// Dependencies: {}
impl < F , T , E > Future for ResponseFuture < F > where F : Future < Output = Result < T , E > > , E : Into < crate :: BoxError > , { type Output = Result < T , crate :: BoxError > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { match self . project () . state . project () { ResponseStateProj :: Called { fut } => { Poll :: Ready (ready ! (fut . poll (cx)) . map_err (Into :: into)) } ResponseStateProj :: Overloaded => Poll :: Ready (Err (Overloaded :: new () . into ())) , } } }
};
}
