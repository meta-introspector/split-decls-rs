macro_rules! deps {
    () => {
        LittleEndianWriteExt!();
    };
}

macro_rules! impl_347 {
    () => {
        deps!();
        impl < W : Write + ? Sized > LittleEndianWriteExt for W { }
    };
}

impl_347!()