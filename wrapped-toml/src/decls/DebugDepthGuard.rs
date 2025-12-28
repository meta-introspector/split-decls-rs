macro_rules! DebugDepthGuard {
    () => {
        pub (crate) struct DebugDepthGuard { depth : usize , inc : bool , }
    };
}

DebugDepthGuard!()