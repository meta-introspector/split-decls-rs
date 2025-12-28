macro_rules! deps {
    () => {
        MakeVisitor!();
        MakeExtMarker!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl < T , M > crate :: sealed :: Sealed < MakeExtMarker < T > > for M where M : MakeVisitor < T > + Sized { }
    };
}

impl_49!()