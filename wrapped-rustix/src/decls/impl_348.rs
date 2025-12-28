macro_rules! deps {
    () => {
        IoringRestrictionOp!();
    };
}

macro_rules! impl_348 {
    () => {
        deps!();
        impl Default for IoringRestrictionOp { # [inline] fn default () -> Self { Self :: RegisterOp } }
    };
}

impl_348!();