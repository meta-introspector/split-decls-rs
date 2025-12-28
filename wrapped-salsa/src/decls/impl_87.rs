macro_rules! deps {
    () => {
        HasStorage!();
        DatabaseImpl!();
        Storage!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        unsafe impl HasStorage for DatabaseImpl { # [inline (always)] fn storage (& self) -> & Storage < Self > { & self . storage } # [inline (always)] fn storage_mut (& mut self) -> & mut Storage < Self > { & mut self . storage } }
    };
}

impl_87!()