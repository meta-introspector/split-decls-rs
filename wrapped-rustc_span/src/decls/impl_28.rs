macro_rules! deps {
    () => {
        RealFileName!();
        FileName!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl From < PathBuf > for FileName { fn from (p : PathBuf) -> Self { FileName :: Real (RealFileName :: LocalPath (p)) } }
    };
}

impl_28!()