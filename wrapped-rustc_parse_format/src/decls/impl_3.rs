macro_rules! deps {
    () => {
        Position!();
        Argument!();
        FormatSpec!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl < 'input > Argument < 'input > { pub fn is_identifier (& self) -> bool { matches ! (self . position , Position :: ArgumentNamed (_)) && self . format == FormatSpec :: default () } }
    };
}

impl_3!()