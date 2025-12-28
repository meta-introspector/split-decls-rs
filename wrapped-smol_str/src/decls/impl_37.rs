macro_rules! deps {
    () => {
        Repr!();
        SmolStr!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl From < Arc < str > > for SmolStr { # [inline] fn from (s : Arc < str >) -> SmolStr { let repr = Repr :: new_on_stack (s . as_ref ()) . unwrap_or (Repr :: Heap (s)) ; Self (repr) } }
    };
}

impl_37!()