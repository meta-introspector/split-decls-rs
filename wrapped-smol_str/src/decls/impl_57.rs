macro_rules! deps {
    () => {
        SmolStr!();
        ToSmolStr!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < T > ToSmolStr for T where T : fmt :: Display + ? Sized , { fn to_smolstr (& self) -> SmolStr { format_smolstr ! ("{}" , self) } }
    };
}

impl_57!();