macro_rules! deps {
    () => {
        Mutex!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < T : ? Sized + Default > Default for Mutex < T > { fn default () -> Mutex < T > { Mutex :: new (Default :: default ()) } }
    };
}

impl_14!();