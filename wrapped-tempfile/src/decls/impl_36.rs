macro_rules! deps {
    () => {
        TempPath!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl Drop for TempPath { fn drop (& mut self) { if ! self . disable_cleanup { let _ = fs :: remove_file (& self . path) ; } } }
    };
}

impl_36!()