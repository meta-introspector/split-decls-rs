macro_rules! deps {
    () => {
        DebugDepthGuard!();
    };
}

macro_rules! impl_205 {
    () => {
        deps!();
        impl Drop for DebugDepthGuard { fn drop (& mut self) { if self . inc { DEBUG_DEPTH . exit_unchecked () ; } } }
    };
}

impl_205!();