macro_rules! deps {
    () => {
        MakeVisitor!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < T , V , F > MakeVisitor < T > for F where F : Fn (T) -> V , V : Visit , { type Visitor = V ; fn make_visitor (& self , target : T) -> Self :: Visitor { (self) (target) } }
    };
}

impl_46!()