macro_rules! deps {
    () => {
        MakeOutput!();
        MakeVisitor!();
        VisitOutput!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < T , Out , M > MakeOutput < T , Out > for M where M : MakeVisitor < T > , M :: Visitor : VisitOutput < Out > , { }
    };
}

impl_48!()