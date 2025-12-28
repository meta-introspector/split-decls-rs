macro_rules! deps {
    () => {
        Sha1Core!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl Default for Sha1Core { # [inline] fn default () -> Self { Self { h : H0 , block_len : 0 , } } }
    };
}

impl_9!();