macro_rules! deps {
    () => {
        FatalError!();
    };
}

macro_rules! impl_226 {
    () => {
        deps!();
        impl FatalError { pub fn raise (self) -> ! { std :: panic :: resume_unwind (Box :: new (FatalErrorMarker)) } }
    };
}

impl_226!();