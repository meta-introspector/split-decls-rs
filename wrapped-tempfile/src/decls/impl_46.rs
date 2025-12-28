macro_rules! deps {
    () => {
        NamedTempFile!();
        PersistError!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < F > From < PersistError < F > > for NamedTempFile < F > { # [inline] fn from (error : PersistError < F >) -> NamedTempFile < F > { error . file } }
    };
}

impl_46!();