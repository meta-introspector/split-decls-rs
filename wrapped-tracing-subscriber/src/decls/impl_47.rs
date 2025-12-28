macro_rules! deps {
    () => {
        VisitOutput!();
        MakeVisitor!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl < T , Out , M > crate :: sealed :: Sealed < (T , Out) > for M where M : MakeVisitor < T > , M :: Visitor : VisitOutput < Out > , { }
    };
}

impl_47!();