// Generated macro for impl_183 (impl)
macro_rules! Depcrate_filter_thenimpl_183 {
() => {
// Module: crate::filter::then
// Provides: {"impl_183"}
// Dependencies: {}
impl < T , F > Future for State < T , F > where T : TryFuture , F : Func < T :: Ok > , F :: Output : Future + Send , { type Output = Result < (< F :: Output as Future > :: Output ,) , T :: Error > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { loop { match self . as_mut () . project () { StateProj :: First (first , second) => { let ex1 = ready ! (first . try_poll (cx)) ? ; let fut2 = second . call (ex1) ; self . set (State :: Second (fut2)) ; } StateProj :: Second (second) => { let ex2 = (ready ! (second . poll (cx)) ,) ; self . set (State :: Done) ; return Poll :: Ready (Ok (ex2)) ; } StateProj :: Done => panic ! ("polled after complete") , } } } }
};
}
