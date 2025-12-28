macro_rules! deps {
    () => {
        PersistError!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < F > From < PersistError < F > > for io :: Error { # [inline] fn from (error : PersistError < F >) -> io :: Error { error . error } }
    };
}

impl_45!()