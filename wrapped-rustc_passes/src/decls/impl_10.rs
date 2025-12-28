macro_rules! deps {
    () => {
        ProcMacroKind!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl IntoDiagArg for ProcMacroKind { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> rustc_errors :: DiagArgValue { match self { ProcMacroKind :: Attribute => "attribute proc macro" , ProcMacroKind :: Derive => "derive proc macro" , ProcMacroKind :: FunctionLike => "function-like proc macro" , } . into_diag_arg (& mut None) } }
    };
}

impl_10!();