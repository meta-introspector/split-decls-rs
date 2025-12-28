macro_rules! deps {
    () => {
        QueryStack!();
        ActiveQuery!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl ops :: Deref for QueryStack { type Target = [ActiveQuery] ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . stack [.. self . len] } }
    };
}

impl_16!();