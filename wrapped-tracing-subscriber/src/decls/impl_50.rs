macro_rules! deps {
    () => {
        MakeExtMarker!();
        MakeExt!();
        MakeVisitor!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl < T , M > MakeExt < T > for M where M : MakeVisitor < T > + Sized , M : crate :: sealed :: Sealed < MakeExtMarker < T > > , { }
    };
}

impl_50!()