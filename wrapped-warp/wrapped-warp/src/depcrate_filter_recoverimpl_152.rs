// Generated macro for impl_152 (impl)
macro_rules! Depcrate_filter_recoverimpl_152 {
() => {
// Module: crate::filter::recover
// Provides: {"impl_152"}
// Dependencies: {}
impl < T , F > Future for RecoverFuture < T , F > where T : Filter , F : Func < T :: Error > , F :: Output : TryFuture + Send , < F :: Output as TryFuture > :: Error : IsReject , { type Output = Result < (Either < T :: Extract , (< F :: Output as TryFuture > :: Ok ,) > ,) , < F :: Output as TryFuture > :: Error , > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { loop { let pin = self . as_mut () . project () ; let (err , second) = match pin . state . project () { StateProj :: First (first , second) => match ready ! (first . try_poll (cx)) { Ok (ex) => return Poll :: Ready (Ok ((Either :: A (ex) ,))) , Err (err) => (err , second) , } , StateProj :: Second (second) => { let ex2 = match ready ! (second . try_poll (cx)) { Ok (ex2) => Ok ((Either :: B ((ex2 ,)) ,)) , Err (e) => Err (e) , } ; self . set (RecoverFuture { state : State :: Done , .. * self }) ; return Poll :: Ready (ex2) ; } StateProj :: Done => panic ! ("polled after complete") , } ; pin . original_path_index . reset_path () ; let fut2 = second . call (err) ; self . set (RecoverFuture { state : State :: Second (fut2) , .. * self }) ; } } }
};
}
