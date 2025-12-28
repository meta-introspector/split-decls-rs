macro_rules! deps {
    () => {
        QueryStack!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl ops :: DerefMut for QueryStack { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . stack [.. self . len] } }
    };
}

impl_17!();