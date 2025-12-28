macro_rules! deps {
    () => {
        Level!();
        EventArgs!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl EventArgs { pub (crate) fn level (& self , default : Level) -> Level { self . level . clone () . unwrap_or (default) } }
    };
}

impl_5!()