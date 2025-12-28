macro_rules! deps {
    () => {
        VisitDelimited!();
        Delimited!();
        MakeVisitor!();
        VisitFmt!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < D , V , T > MakeVisitor < T > for Delimited < D , V > where D : AsRef < str > + Clone , V : MakeVisitor < T > , V :: Visitor : VisitFmt , { type Visitor = VisitDelimited < D , V :: Visitor > ; fn make_visitor (& self , target : T) -> Self :: Visitor { let inner = self . inner . make_visitor (target) ; VisitDelimited :: new (self . delimiter . clone () , inner) } }
    };
}

impl_15!()