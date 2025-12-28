macro_rules! deps {
    () => {
        IoringOp!();
    };
}

macro_rules! impl_346 {
    () => {
        deps!();
        impl Default for IoringOp { # [inline] fn default () -> Self { Self :: Nop } }
    };
}

impl_346!()