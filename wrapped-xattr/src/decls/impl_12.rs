macro_rules! deps {
    () => {
        FileExt!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl FileExt for File { }
    };
}

impl_12!()