macro_rules! deps {
    () => {
        RecordFields!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < F > RecordFields for & F where F : RecordFields , { fn record (& self , visitor : & mut dyn Visit) { F :: record (* self , visitor) } }
    };
}

impl_45!()