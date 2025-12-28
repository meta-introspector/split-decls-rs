macro_rules! deps {
    () => {
        SalsaAsRef!();
    };
}

macro_rules! impl_241 {
    () => {
        deps!();
        impl < T > SalsaAsRef for Option < T > { type AsRef < 'a > = Option < & 'a T > where Self : 'a ; fn as_ref (& self) -> Self :: AsRef < '_ > { self . as_ref () } }
    };
}

impl_241!();