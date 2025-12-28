macro_rules! deps {
    () => {
        EventReceiver!();
        Source!();
        ValidateWhitespace!();
    };
}

macro_rules! impl_184 {
    () => {
        deps!();
        impl < 'r , 's > ValidateWhitespace < 'r , 's > { pub fn new (receiver : & 'r mut dyn EventReceiver , source : Source < 's >) -> Self { Self { receiver , source } } }
    };
}

impl_184!()