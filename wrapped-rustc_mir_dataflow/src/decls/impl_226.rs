macro_rules! deps {
    () => {
        PeekCallKind!();
    };
}

macro_rules! impl_226 {
    () => {
        deps!();
        impl PeekCallKind { fn from_arg_ty (arg : Ty < '_ >) -> Self { match arg . kind () { ty :: Ref (_ , _ , _) => PeekCallKind :: ByRef , _ => PeekCallKind :: ByVal , } } }
    };
}

impl_226!()