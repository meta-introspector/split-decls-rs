macro_rules! deps {
    () => {
        Level!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl IntoDiagArg for Level { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Borrowed (self . to_cmd_flag ())) } }
    };
}

impl_146!();