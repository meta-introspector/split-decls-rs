macro_rules! deps {
    () => {
        RecordFields!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl RecordFields for Event < '_ > { fn record (& self , visitor : & mut dyn Visit) { Event :: record (self , visitor) } }
    };
}

impl_39!();