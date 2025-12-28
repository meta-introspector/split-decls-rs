macro_rules! deps {
    () => {
        Map!();
        RawMap!();
        Downcast!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < A : ? Sized + Downcast > Default for Map < A > { # [inline] fn default () -> Map < A > { Map { raw : RawMap :: with_hasher (Default :: default ()) } } }
    };
}

impl_9!()