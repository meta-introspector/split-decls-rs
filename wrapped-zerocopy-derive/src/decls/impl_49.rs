macro_rules! deps {
    () => {
        UnsupportedReprError!();
        FromAttrsError!();
        FromRawReprsError!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl From < FromRawReprsError < UnsupportedReprError > > for FromAttrsError { fn from (err : FromRawReprsError < UnsupportedReprError >) -> FromAttrsError { FromAttrsError :: FromRawReprs (err) } }
    };
}

impl_49!()