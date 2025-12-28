macro_rules! deps {
    () => {
        TempPath!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl Deref for TempPath { type Target = Path ; fn deref (& self) -> & Path { & self . path } }
    };
}

impl_37!();