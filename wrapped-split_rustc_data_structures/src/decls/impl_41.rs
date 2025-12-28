macro_rules! deps {
    () => {
        OnDrop!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl < F : FnOnce () > OnDrop < F > { # [doc = " Disables on-drop call."] # [inline] pub fn disable (mut self) { self . 0 . take () ; } }
    };
}

impl_41!()