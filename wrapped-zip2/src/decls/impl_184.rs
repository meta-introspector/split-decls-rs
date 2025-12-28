macro_rules! deps {
    () => {
        System!();
    };
}

macro_rules! impl_184 {
    () => {
        deps!();
        impl From < u8 > for System { fn from (system : u8) -> Self { match system { 0 => Self :: Dos , 3 => Self :: Unix , _ => Self :: Unknown , } } }
    };
}

impl_184!()