macro_rules! deps {
    () => {
        Args!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl Args { fn new () -> Self { Self :: Split (Default :: default ()) } fn as_slice (& self) -> & [String] { match self { Self :: Joined (j) => j . inner . as_slice () , Self :: Split (v) => v . as_slice () , } } fn into_vec (self) -> Vec < String > { match self { Self :: Joined (j) => j . inner , Self :: Split (v) => v , } } }
    };
}

impl_16!()