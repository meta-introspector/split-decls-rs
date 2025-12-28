macro_rules! deps {
    () => {
        HSTRING!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl Clone for HSTRING { fn clone (& self) -> Self { if let Some (header) = self . as_header () { Self (header . duplicate ()) } else { Self :: new () } } }
    };
}

impl_23!()