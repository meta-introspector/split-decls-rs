macro_rules! deps {
    () => {
        FromAttrsError!();
        FromRawReprsError!();
        UnsupportedReprError!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl From < FromRawReprsError < UnsupportedReprError > > for FromAttrsError { fn from (err : FromRawReprsError < UnsupportedReprError >) -> FromAttrsError { FromAttrsError :: FromRawReprs (err) } }
    };
}

impl_49!();