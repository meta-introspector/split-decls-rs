macro_rules! deps {
    () => {
        GitDaemon!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl Drop for GitDaemon { fn drop (& mut self) { self . child . kill () . ok () ; } }
    };
}

impl_2!();