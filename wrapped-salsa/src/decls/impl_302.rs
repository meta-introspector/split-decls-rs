macro_rules! deps {
    () => {
        CoordinateDrop!();
    };
}

macro_rules! impl_302 {
    () => {
        deps!();
        impl Drop for CoordinateDrop { fn drop (& mut self) { self . 0 . cvar . notify_all () ; } }
    };
}

impl_302!()