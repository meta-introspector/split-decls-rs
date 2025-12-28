macro_rules! deps {
    () => {
        HSTRING!();
        HStringHeader!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl Drop for HSTRING { fn drop (& mut self) { if let Some (header) = self . as_header () { unsafe { if header . flags & HSTRING_REFERENCE_FLAG == 0 && header . count . release () == 0 { HStringHeader :: free (self . 0) ; } } } } }
    };
}

impl_24!()