// Generated macro for impl_47 (impl)
macro_rules! Depcrate_filter_andimpl_47 {
() => {
// Module: crate::filter::and
// Provides: {"impl_47"}
// Dependencies: {}
impl < T , U > Future for AndFuture < T , U > where T : Filter , U : Filter , < T :: Extract as Tuple > :: HList : Combine < < U :: Extract as Tuple > :: HList > + Send , U :: Error : CombineRejection < T :: Error > , { type Output = Result < CombinedTuples < T :: Extract , U :: Extract > , < U :: Error as CombineRejection < T :: Error > > :: One , > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { self . project () . state . poll (cx) } }
};
}
