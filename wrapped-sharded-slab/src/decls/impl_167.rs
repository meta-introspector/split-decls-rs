macro_rules! deps {
    () => {
        Config!();
        Registration!();
        Tid!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl < C : cfg :: Config > Tid < C > { # [inline] pub (crate) fn current () -> Self { REGISTRATION . try_with (Registration :: current) . unwrap_or_else (| _ | Self :: poisoned ()) } pub (crate) fn is_current (self) -> bool { REGISTRATION . try_with (| r | self == r . current :: < C > ()) . unwrap_or (false) } # [inline (always)] pub fn new (id : usize) -> Self { Self :: from_usize (id) } }
    };
}

impl_167!();