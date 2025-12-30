// Generated macro for impl_135 (impl)
macro_rules! Depcrate_filter_or_elseimpl_135 {
() => {
// Module: crate::filter::or_else
// Provides: {"impl_135"}
// Dependencies: {}
impl < T , F > Future for OrElseFuture < T , F > where T : Filter , F : Func < T :: Error > , F :: Output : TryFuture < Ok = T :: Extract > + Send , { type Output = Result < < F :: Output as TryFuture > :: Ok , < F :: Output as TryFuture > :: Error > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { loop { let pin = self . as_mut () . project () ; let (err , second) = match pin . state . project () { StateProj :: First (first , second) => match ready ! (first . try_poll (cx)) { Ok (ex) => return Poll :: Ready (Ok (ex)) , Err (err) => (err , second) , } , StateProj :: Second (second) => { let ex2 = ready ! (second . try_poll (cx)) ; self . set (OrElseFuture { state : State :: Done , .. * self }) ; return Poll :: Ready (ex2) ; } StateProj :: Done => panic ! ("polled after complete") , } ; pin . original_path_index . reset_path () ; let fut2 = second . call (err) ; self . set (OrElseFuture { state : State :: Second (fut2) , .. * self }) ; } } }
};
}
