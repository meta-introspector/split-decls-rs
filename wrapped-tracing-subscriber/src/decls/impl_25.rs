macro_rules! deps {
    () => {
        MakeVisitor!();
        Messages!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < T , V > MakeVisitor < T > for Messages < V > where V : MakeVisitor < T > , { type Visitor = Messages < V :: Visitor > ; # [inline] fn make_visitor (& self , target : T) -> Self :: Visitor { Messages (self . 0 . make_visitor (target)) } }
    };
}

impl_25!()