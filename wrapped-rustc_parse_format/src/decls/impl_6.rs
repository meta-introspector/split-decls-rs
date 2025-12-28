macro_rules! deps {
    () => {
        Position!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl Position < '_ > { pub fn index (& self) -> Option < usize > { match self { ArgumentIs (i , ..) | ArgumentImplicitlyIs (i) => Some (* i) , _ => None , } } }
    };
}

impl_6!()