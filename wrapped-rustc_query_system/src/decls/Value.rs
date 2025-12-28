macro_rules! deps {
    () => {
        DepContext!();
        CycleError!();
    };
}

macro_rules! Value {
    () => {
        deps!();
        pub trait Value < Tcx : DepContext > : Sized { fn from_cycle_error (tcx : Tcx , cycle_error : & CycleError , guar : ErrorGuaranteed) -> Self ; }
    };
}

Value!()