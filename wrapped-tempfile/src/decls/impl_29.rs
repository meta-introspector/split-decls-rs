macro_rules! deps {
    () => {
        PathPersistError!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl From < PathPersistError > for io :: Error { # [inline] fn from (error : PathPersistError) -> io :: Error { error . error } }
    };
}

impl_29!();