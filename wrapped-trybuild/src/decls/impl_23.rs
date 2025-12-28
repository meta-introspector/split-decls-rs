macro_rules! deps {
    () => {
        TestCases!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        # [doc (hidden)] impl Drop for TestCases { fn drop (& mut self) { if ! thread :: panicking () { self . runner . borrow_mut () . run () ; } } }
    };
}

impl_23!()