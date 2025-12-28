macro_rules! deps {
    () => {
        System!();
    };
}

macro_rules! impl_185 {
    () => {
        deps!();
        impl From < System > for u8 { fn from (system : System) -> Self { match system { System :: Dos => 0 , System :: Unix => 3 , System :: Unknown => 4 , } } }
    };
}

impl_185!()