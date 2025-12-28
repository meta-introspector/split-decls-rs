macro_rules! impl_483 {
    () => {
        impl IntoDiagArg for PanicStrategy { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Owned (self . desc () . to_string ())) } }
    };
}

impl_483!()