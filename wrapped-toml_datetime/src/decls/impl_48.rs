macro_rules! deps {
    () => {
        DatetimeOrTable!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < 'm , 'de > DatetimeOrTable < 'm , 'de > { fn new (key : & 'm mut Option < alloc :: borrow :: Cow < 'de , str > >) -> Self { * key = None ; Self { key } } }
    };
}

impl_48!();