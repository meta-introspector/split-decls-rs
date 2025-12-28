macro_rules! deps {
    () => {
        CompilerError!();
    };
}

macro_rules! impl_277 {
    () => {
        deps!();
        impl < T > Display for CompilerError < T > where T : Display , { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { match self { CompilerError :: Failed => write ! (f , "Compilation Failed") , CompilerError :: Interrupted (reason) => write ! (f , "Compilation Interrupted: {reason}") , CompilerError :: Skipped => write ! (f , "Compilation Skipped") , } } }
    };
}

impl_277!()