// Generated macro for impl_44 (impl)
macro_rules! Depcrate_filter_andimpl_44 {
() => {
// Module: crate::filter::and
// Provides: {"impl_44"}
// Dependencies: {}
impl < T , U > FilterBase for And < T , U > where T : Filter , T :: Extract : Send , U : Filter + Clone + Send , < T :: Extract as Tuple > :: HList : Combine < < U :: Extract as Tuple > :: HList > + Send , CombinedTuples < T :: Extract , U :: Extract > : Send , U :: Error : CombineRejection < T :: Error > , { type Extract = CombinedTuples < T :: Extract , U :: Extract > ; type Error = < U :: Error as CombineRejection < T :: Error > > :: One ; type Future = AndFuture < T , U > ; fn filter (& self , _ : Internal) -> Self :: Future { AndFuture { state : State :: First (self . first . filter (Internal) , self . second . clone ()) , } } }
};
}
