macro_rules! deps {
    () => {
        FileExt!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl FileExt for File { }
    };
}

impl_19!();