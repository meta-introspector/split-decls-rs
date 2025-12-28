macro_rules! deps {
    () => {
        SalsaAsRef!();
    };
}

macro_rules! impl_242 {
    () => {
        deps!();
        impl < T , E > SalsaAsRef for Result < T , E > { type AsRef < 'a > = Result < & 'a T , & 'a E > where Self : 'a ; fn as_ref (& self) -> Self :: AsRef < '_ > { self . as_ref () } }
    };
}

impl_242!()