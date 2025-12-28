macro_rules! deps {
    () => {
        SalsaAsDeref!();
    };
}

macro_rules! impl_244 {
    () => {
        deps!();
        impl < T : Deref > SalsaAsDeref for Option < T > { type AsDeref < 'a > = Option < & 'a T :: Target > where Self : 'a ; fn as_deref (& self) -> Self :: AsDeref < '_ > { self . as_deref () } }
    };
}

impl_244!();