// Generated macro for impl_398 (impl)
macro_rules! Depcrate_valuesimpl_398 {
() => {
// Module: crate::values
// Provides: {"impl_398"}
// Dependencies: {}
impl < Tcx : DepContext , T > Value < Tcx > for T { default fn from_cycle_error (tcx : Tcx , cycle_error : & CycleError , _guar : ErrorGuaranteed) -> T { tcx . sess () . dcx () . abort_if_errors () ; panic ! ("<{} as Value>::from_cycle_error called without errors: {:#?}" , std :: any :: type_name ::< T > () , cycle_error . cycle ,) ; } }
};
}
