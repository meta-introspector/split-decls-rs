macro_rules! deps {
    () => {
        Target!();
    };
}

macro_rules! impl_543 {
    () => {
        deps!();
        impl DerefMut for Target { # [inline] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . options } }
    };
}

impl_543!();