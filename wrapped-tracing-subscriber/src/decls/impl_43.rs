macro_rules! deps {
    () => {
        RecordFields!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl RecordFields for Record < '_ > { fn record (& self , visitor : & mut dyn Visit) { Record :: record (self , visitor) } }
    };
}

impl_43!()