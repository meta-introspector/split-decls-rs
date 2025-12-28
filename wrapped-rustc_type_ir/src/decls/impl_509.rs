macro_rules! deps {
    () => {
        HasErrorVisitor!();
        Interner!();
        TypeVisitor!();
    };
}

macro_rules! impl_509 {
    () => {
        deps!();
        impl < I : Interner > TypeVisitor < I > for HasErrorVisitor { type Result = ControlFlow < I :: ErrorGuaranteed > ; fn visit_error (& mut self , guar : < I as Interner > :: ErrorGuaranteed) -> Self :: Result { ControlFlow :: Break (guar) } }
    };
}

impl_509!()