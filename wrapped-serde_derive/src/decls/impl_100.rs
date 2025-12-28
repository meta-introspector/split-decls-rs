macro_rules! deps {
    () => {
        Ctxt!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl Drop for Ctxt { fn drop (& mut self) { if ! thread :: panicking () && self . errors . borrow () . is_some () { panic ! ("forgot to check for errors") ; } } }
    };
}

impl_100!()