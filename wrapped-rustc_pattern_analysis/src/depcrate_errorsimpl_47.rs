// Generated macro for impl_47 (impl)
macro_rules! Depcrate_errorsimpl_47 {
() => {
// Module: crate::errors
// Provides: {"impl_47"}
// Dependencies: {}
impl Uncovered { pub fn new < 'p , 'tcx > (span : Span , cx : & RustcPatCtxt < 'p , 'tcx > , witnesses : Vec < WitnessPat < 'p , 'tcx > > ,) -> Self where 'tcx : 'p , { let witness_1 = cx . print_witness_pat (witnesses . get (0) . unwrap ()) ; Self { span , count : witnesses . len () , witness_2 : witnesses . get (1) . map (| w | cx . print_witness_pat (w)) . unwrap_or_default () , witness_3 : witnesses . get (2) . map (| w | cx . print_witness_pat (w)) . unwrap_or_default () , witness_1 , remainder : witnesses . len () . saturating_sub (3) , } } }
};
}
