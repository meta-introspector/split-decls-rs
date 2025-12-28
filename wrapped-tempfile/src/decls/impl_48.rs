macro_rules! deps {
    () => {
        PersistError!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < F > error :: Error for PersistError < F > { fn source (& self) -> Option < & (dyn error :: Error + 'static) > { Some (& self . error) } }
    };
}

impl_48!();