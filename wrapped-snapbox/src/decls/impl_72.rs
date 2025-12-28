macro_rules! deps {
    () => {
        OrdRange!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl From < std :: ops :: Range < usize > > for OrdRange { fn from (other : std :: ops :: Range < usize >) -> Self { Self { start : other . start , end : other . end , } } }
    };
}

impl_72!()