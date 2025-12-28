macro_rules! deps {
    () => {
        RecordFields!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl RecordFields for Attributes < '_ > { fn record (& self , visitor : & mut dyn Visit) { Attributes :: record (self , visitor) } }
    };
}

impl_41!()