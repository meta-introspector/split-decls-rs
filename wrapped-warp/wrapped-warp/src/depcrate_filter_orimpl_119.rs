// Generated macro for impl_119 (impl)
macro_rules! Depcrate_filter_orimpl_119 {
() => {
// Module: crate::filter::or
// Provides: {"impl_119"}
// Dependencies: {}
impl < T , U > Future for EitherFuture < T , U > where T : Filter , U : Filter , U :: Error : CombineRejection < T :: Error > , { type Output = Result < (Either < T :: Extract , U :: Extract > ,) , Combined < U :: Error , T :: Error > > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { loop { let pin = self . as_mut () . project () ; let (err1 , fut2) = match pin . state . project () { StateProj :: First (first , second) => match ready ! (first . try_poll (cx)) { Ok (ex1) => { return Poll :: Ready (Ok ((Either :: A (ex1) ,))) ; } Err (e) => { pin . original_path_index . reset_path () ; (e , second . filter (Internal)) } } , StateProj :: Second (err1 , second) => { let ex2 = match ready ! (second . try_poll (cx)) { Ok (ex2) => Ok ((Either :: B (ex2) ,)) , Err (e) => { pin . original_path_index . reset_path () ; let err1 = err1 . take () . expect ("polled after complete") ; Err (e . combine (err1)) } } ; self . set (EitherFuture { state : State :: Done , .. * self }) ; return Poll :: Ready (ex2) ; } StateProj :: Done => panic ! ("polled after complete") , } ; self . set (EitherFuture { state : State :: Second (Some (err1) , fut2) , .. * self }) ; } } }
};
}
