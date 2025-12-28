macro_rules! deps {
    () => {
        FromRawReprsError!();
        UnsupportedReprError!();
    };
}

macro_rules! FromAttrsError {
    () => {
        deps!();
        # [doc = " The error returned from [`Repr::from_attrs`]."] # [cfg_attr (test , derive (Copy , Clone , Debug , Eq , PartialEq))] enum FromAttrsError { FromRawReprs (FromRawReprsError < UnsupportedReprError >) , Unrecognized , }
    };
}

FromAttrsError!()