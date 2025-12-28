macro_rules! deps {
    () => {
        FormatSpec!();
        Position!();
        Argument!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl < 'input > Argument < 'input > { pub fn is_identifier (& self) -> bool { matches ! (self . position , Position :: ArgumentNamed (_)) && self . format == FormatSpec :: default () } }
    };
}

impl_3!()