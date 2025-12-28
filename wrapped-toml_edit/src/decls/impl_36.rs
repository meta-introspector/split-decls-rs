macro_rules! deps {
    () => {
        Table!();
        Document!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl < S > std :: ops :: Deref for Document < S > { type Target = Table ; fn deref (& self) -> & Self :: Target { self . as_table () } }
    };
}

impl_36!();