macro_rules! deps {
    () => {
        Fields!();
        LogVisitor!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl < 'a > LogVisitor < 'a > { fn new_for (_event : & 'a Event < 'a > , fields : & 'static Fields) -> Self { Self { target : None , module_path : None , file : None , line : None , fields , } } }
    };
}

impl_66!();