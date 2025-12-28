macro_rules! deps {
    () => {
        Pool!();
        Scope!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl Deref for Scope < '_ , '_ > { type Target = Pool ; fn deref (& self) -> & Self :: Target { self . pool } }
    };
}

impl_32!()