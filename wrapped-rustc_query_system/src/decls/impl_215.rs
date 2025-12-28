macro_rules! deps {
    () => {
        Value!();
        DepContext!();
        CycleError!();
    };
}

macro_rules! impl_215 {
    () => {
        deps!();
        impl < Tcx : DepContext , T > Value < Tcx > for T { default fn from_cycle_error (tcx : Tcx , cycle_error : & CycleError , _guar : ErrorGuaranteed) -> T { tcx . sess () . dcx () . abort_if_errors () ; panic ! ("<{} as Value>::from_cycle_error called without errors: {:#?}" , std :: any :: type_name ::< T > () , cycle_error . cycle ,) ; } }
    };
}

impl_215!();