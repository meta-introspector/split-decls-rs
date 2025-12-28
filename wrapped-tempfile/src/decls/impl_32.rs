macro_rules! deps {
    () => {
        PathPersistError!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl error :: Error for PathPersistError { fn source (& self) -> Option < & (dyn error :: Error + 'static) > { Some (& self . error) } }
    };
}

impl_32!();