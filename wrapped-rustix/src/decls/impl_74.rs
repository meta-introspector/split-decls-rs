macro_rules! impl_74 {
    () => {
        impl PartialEq for EventData { # [inline] fn eq (& self , other : & Self) -> bool { self . u64 () == other . u64 () } }
    };
}

impl_74!();