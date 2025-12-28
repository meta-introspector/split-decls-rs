macro_rules! deps {
    () => {
        BaseNString!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl std :: ops :: Deref for BaseNString { type Target = str ; fn deref (& self) -> & str { self . buf [self . start ..] . as_str () } }
    };
}

impl_11!();