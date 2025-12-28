macro_rules! deps {
    () => {
        CaptureKey!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < 'a , X > CaptureKey < 'a , X > { fn new (delegate : X , key : & 'a mut Option < String >) -> Self { CaptureKey { delegate , key } } }
    };
}

impl_14!();