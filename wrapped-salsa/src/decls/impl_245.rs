macro_rules! deps {
    () => {
        SalsaAsDeref!();
    };
}

macro_rules! impl_245 {
    () => {
        deps!();
        impl < T : Deref , E > SalsaAsDeref for Result < T , E > { type AsDeref < 'a > = Result < & 'a T :: Target , & 'a E > where Self : 'a ; fn as_deref (& self) -> Self :: AsDeref < '_ > { self . as_deref () } }
    };
}

impl_245!();