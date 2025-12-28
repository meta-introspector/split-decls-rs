macro_rules! deps {
    () => {
        DebugDepthGuard!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl Drop for DebugDepthGuard { fn drop (& mut self) { if self . inc { DEBUG_DEPTH . exit_unchecked () ; } } }
    };
}

impl_167!()