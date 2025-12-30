// Generated macro for impl_48 (impl)
macro_rules! Depcrate_filter_andimpl_48 {
() => {
// Module: crate::filter::and
// Provides: {"impl_48"}
// Dependencies: {}
impl < T , TE , U , E > Future for State < T , TE , U > where T : Future < Output = Result < TE , E > > , U : Filter , TE : Tuple , TE :: HList : Combine < < U :: Extract as Tuple > :: HList > + Send , U :: Error : CombineRejection < E > , { type Output = Result < CombinedTuples < TE , U :: Extract > , < U :: Error as CombineRejection < E > > :: One > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { loop { match self . as_mut () . project () { StateProj :: First (first , second) => { let ex1 = ready ! (first . poll (cx)) ? ; let fut2 = second . filter (Internal) ; self . set (State :: Second (Some (ex1) , fut2)) ; } StateProj :: Second (ex1 , second) => { let ex2 = ready ! (second . poll (cx)) ? ; let ex3 = ex1 . take () . unwrap () . combine (ex2) ; self . set (State :: Done) ; return Poll :: Ready (Ok (ex3)) ; } StateProj :: Done => panic ! ("polled after complete") , } } } }
};
}
