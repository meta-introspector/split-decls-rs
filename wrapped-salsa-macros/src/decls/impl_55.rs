macro_rules! deps {
    () => {
        AllowedPersistOptions!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl AllowedPersistOptions { fn allowed (& self) -> bool { matches ! (self , Self :: AllowedIdent | Self :: AllowedValue) } fn allowed_value (& self) -> bool { matches ! (self , Self :: AllowedValue) } }
    };
}

impl_55!();