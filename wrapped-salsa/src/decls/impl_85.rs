macro_rules! deps {
    () => {
        Storage!();
        DatabaseImpl!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl DatabaseImpl { # [doc = " Create a new database; equivalent to `Self::default`."] pub fn new () -> Self { Self :: default () } pub fn storage (& self) -> & Storage < Self > { & self . storage } }
    };
}

impl_85!();