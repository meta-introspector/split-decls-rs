macro_rules! deps {
    () => {
        EntryRef!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl < K , V > EntryRef < K , V > for (& K , & V) { fn get_ref (& self) -> (& K , & V) { * self } }
    };
}

impl_63!();