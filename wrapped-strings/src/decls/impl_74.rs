macro_rules! deps {
    () => {
        HStringHeader!();
        HStringBuilder!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl Drop for HStringBuilder { fn drop (& mut self) { unsafe { HStringHeader :: free (self . 0) ; } } }
    };
}

impl_74!();