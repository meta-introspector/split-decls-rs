macro_rules! deps {
    () => {
        Ident!();
    };
}

macro_rules! impl_178 {
    () => {
        deps!();
        impl PartialEq for Ident { # [inline] fn eq (& self , rhs : & Self) -> bool { self . name == rhs . name && self . span . eq_ctxt (rhs . span) } }
    };
}

impl_178!()