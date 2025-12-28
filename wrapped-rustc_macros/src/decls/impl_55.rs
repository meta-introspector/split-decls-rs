macro_rules! deps {
    () => {
        Applicability!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl quote :: ToTokens for Applicability { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . extend (match self { Applicability :: MachineApplicable => { quote ! { rustc_errors :: Applicability :: MachineApplicable } } Applicability :: MaybeIncorrect => { quote ! { rustc_errors :: Applicability :: MaybeIncorrect } } Applicability :: HasPlaceholders => { quote ! { rustc_errors :: Applicability :: HasPlaceholders } } Applicability :: Unspecified => { quote ! { rustc_errors :: Applicability :: Unspecified } } }) ; } }
    };
}

impl_55!();