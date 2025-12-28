macro_rules! deps {
    () => {
        FnContext!();
    };
}

macro_rules! impl_350 {
    () => {
        deps!();
        impl FnContext { # [doc = " Returns `true` if the closure was called from a different thread"] # [doc = " than it was provided from."] # [inline] pub fn migrated (& self) -> bool { self . migrated } }
    };
}

impl_350!()