macro_rules! deps {
    () => {
        JodChild!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl ops :: Deref for JodChild { type Target = std :: process :: Child ; fn deref (& self) -> & std :: process :: Child { & self . 0 } }
    };
}

impl_83!();