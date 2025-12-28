macro_rules! deps {
    () => {
        OnDrop!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl < F : FnOnce () > Drop for OnDrop < F > { # [inline] fn drop (& mut self) { if let Some (f) = self . 0 . take () { f () ; } } }
    };
}

impl_42!()