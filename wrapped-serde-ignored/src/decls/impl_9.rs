macro_rules! deps {
    () => {
        Wrap!();
        Path!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < 'a , 'b , X , F > Wrap < 'a , 'b , X , F > { fn new (delegate : X , callback : & 'b mut F , path : & 'a Path < 'a >) -> Self { Wrap { delegate , callback , path , } } }
    };
}

impl_9!();