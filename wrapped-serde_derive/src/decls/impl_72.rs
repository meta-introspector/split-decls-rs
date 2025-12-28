macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl From < & LitStr > for Name { fn from (lit : & LitStr) -> Self { Name { value : lit . value () , span : lit . span () , } } }
    };
}

impl_72!();