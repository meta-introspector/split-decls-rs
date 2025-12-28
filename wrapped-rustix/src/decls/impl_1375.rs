macro_rules! deps {
    () => {
        CpuSet!();
    };
}

macro_rules! impl_1375 {
    () => {
        deps!();
        impl Eq for CpuSet { }
    };
}

impl_1375!()