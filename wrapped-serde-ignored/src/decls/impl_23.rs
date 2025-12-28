macro_rules! deps {
    () => {
        Path!();
        SeqAccess!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < 'a , 'b , X , F > SeqAccess < 'a , 'b , X , F > { fn new (delegate : X , callback : & 'b mut F , path : & 'a Path < 'a >) -> Self { SeqAccess { delegate , callback , path , index : 0 , } } }
    };
}

impl_23!();