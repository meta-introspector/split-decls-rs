macro_rules! deps {
    () => {
        JodChild!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl Drop for JodChild { fn drop (& mut self) { _ = self . 0 . kill () ; _ = self . 0 . wait () ; } }
    };
}

impl_29!()