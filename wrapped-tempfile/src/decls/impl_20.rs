macro_rules! deps {
    () => {
        PathError!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl error :: Error for PathError { fn source (& self) -> Option < & (dyn error :: Error + 'static) > { self . err . source () } }
    };
}

impl_20!()