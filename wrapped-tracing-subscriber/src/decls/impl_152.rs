macro_rules! deps {
    () => {
        TryInitError!();
    };
}

macro_rules! impl_152 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl Error for TryInitError { fn source (& self) -> Option < & (dyn Error + 'static) > { self . inner . source () } }
    };
}

impl_152!()