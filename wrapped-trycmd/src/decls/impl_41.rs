macro_rules! deps {
    () => {
        TestCases!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        # [doc (hidden)] impl Drop for TestCases { fn drop (& mut self) { if ! self . has_run . get () && ! std :: thread :: panicking () { self . run () ; } } }
    };
}

impl_41!()