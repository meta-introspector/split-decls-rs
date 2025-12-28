macro_rules! deps {
    () => {
        DebugDepthGuard!();
    };
}

macro_rules! impl_207 {
    () => {
        deps!();
        impl core :: ops :: Deref for DebugDepthGuard { type Target = usize ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . depth } }
    };
}

impl_207!();