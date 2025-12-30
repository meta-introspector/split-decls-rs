// Generated macro for impl_89 (impl)
macro_rules! Depcrate_filter_mapimpl_89 {
() => {
// Module: crate::filter::map
// Provides: {"impl_89"}
// Dependencies: {}
impl < T , F > Future for MapFuture < T , F > where T : Filter , F : Func < T :: Extract > , { type Output = Result < (F :: Output ,) , T :: Error > ; # [inline] fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let pin = self . project () ; match ready ! (pin . extract . try_poll (cx)) { Ok (ex) => { let ex = (pin . callback . call (ex) ,) ; Poll :: Ready (Ok (ex)) } Err (err) => Poll :: Ready (Err (err)) , } } }
};
}
