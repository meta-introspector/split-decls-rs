macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < T , C : cfg :: Config > std :: ops :: Deref for Entry < '_ , T , C > { type Target = T ; fn deref (& self) -> & Self :: Target { self . value () } }
    };
}

impl_21!()