macro_rules! deps {
    () => {
        FromAttrsError!();
        UnrecognizedReprError!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl From < UnrecognizedReprError > for FromAttrsError { fn from (_err : UnrecognizedReprError) -> FromAttrsError { FromAttrsError :: Unrecognized } }
    };
}

impl_50!()