macro_rules! deps {
    () => {
        TempPath!();
        PathPersistError!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl From < PathPersistError > for TempPath { # [inline] fn from (error : PathPersistError) -> TempPath { error . path } }
    };
}

impl_30!();