macro_rules! deps {
    () => {
        TempDir!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl Drop for TempDir { fn drop (& mut self) { if ! self . disable_cleanup { let _ = remove_dir_all (self . path ()) ; } } }
    };
}

impl_9!();