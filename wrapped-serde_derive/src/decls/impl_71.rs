macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl From < & Ident > for Name { fn from (ident : & Ident) -> Self { Name { value : ident . to_string () , span : ident . span () , } } }
    };
}

impl_71!()