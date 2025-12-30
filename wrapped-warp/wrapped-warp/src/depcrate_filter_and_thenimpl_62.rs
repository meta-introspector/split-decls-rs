// Generated macro for impl_62 (impl)
macro_rules! Depcrate_filter_and_thenimpl_62 {
() => {
// Module: crate::filter::and_then
// Provides: {"impl_62"}
// Dependencies: {}
impl < T , F > Future for State < T , F > where T : TryFuture , F : Func < T :: Ok > , F :: Output : TryFuture + Send , < F :: Output as TryFuture > :: Error : CombineRejection < T :: Error > , { type Output = Result < (< F :: Output as TryFuture > :: Ok ,) , < < F :: Output as TryFuture > :: Error as CombineRejection < T :: Error > > :: One , > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { loop { match self . as_mut () . project () { StateProj :: First (first , second) => { let ex1 = ready ! (first . try_poll (cx)) ? ; let fut2 = second . call (ex1) ; self . set (State :: Second (fut2)) ; } StateProj :: Second (second) => { let ex2 = match ready ! (second . try_poll (cx)) { Ok (item) => Ok ((item ,)) , Err (err) => Err (From :: from (err)) , } ; self . set (State :: Done) ; return Poll :: Ready (ex2) ; } StateProj :: Done => panic ! ("polled after complete") , } } } }
};
}
