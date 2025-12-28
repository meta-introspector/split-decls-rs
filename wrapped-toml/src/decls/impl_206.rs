macro_rules! deps {
    () => {
        DebugDepthGuard!();
    };
}

macro_rules! impl_206 {
    () => {
        deps!();
        impl AsRef < usize > for DebugDepthGuard { # [inline (always)] fn as_ref (& self) -> & usize { & self . depth } }
    };
}

impl_206!();