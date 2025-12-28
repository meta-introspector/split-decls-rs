macro_rules! deps {
    () => {
        Entry!();
        Config!();
    };
}

macro_rules! impl_189 {
    () => {
        deps!();
        impl < T , C : cfg :: Config > std :: ops :: Deref for Entry < '_ , T , C > { type Target = T ; fn deref (& self) -> & Self :: Target { self . value () } }
    };
}

impl_189!()