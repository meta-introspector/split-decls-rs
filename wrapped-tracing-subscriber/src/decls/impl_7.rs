macro_rules! deps {
    () => {
        Alt!();
        MakeVisitor!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl < T , V > MakeVisitor < T > for Alt < V > where V : MakeVisitor < T > , { type Visitor = Alt < V :: Visitor > ; # [inline] fn make_visitor (& self , target : T) -> Self :: Visitor { Alt (self . 0 . make_visitor (target)) } }
    };
}

impl_7!()