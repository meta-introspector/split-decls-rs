// Generated macro for impl_140 (impl)
macro_rules! Depcrate_discover_listimpl_140 {
() => {
// Module: crate::discover::list
// Provides: {"impl_140"}
// Dependencies: {}
impl < T , U > Stream for ServiceList < T > where T : IntoIterator < Item = U > , { type Item = Result < Change < usize , U > , Infallible > ; fn poll_next (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { match self . project () . inner . next () { Some ((i , service)) => Poll :: Ready (Some (Ok (Change :: Insert (i , service)))) , None => Poll :: Ready (None) , } } }
};
}
