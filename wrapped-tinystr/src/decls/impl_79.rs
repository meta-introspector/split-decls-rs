macro_rules! deps {
    () => {
        UnvalidatedTinyAsciiStr!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl < const N : usize > databake :: Bake for UnvalidatedTinyAsciiStr < N > { fn bake (& self , env : & databake :: CrateEnv) -> databake :: TokenStream { match self . try_into_tinystr () { Ok (tiny) => { let tiny = tiny . bake (env) ; databake :: quote ! { # tiny . to_unvalidated () } } Err (_) => { let bytes = self . 0 . bake (env) ; env . insert ("tinystr") ; databake :: quote ! { tinystr :: UnvalidatedTinyAsciiStr :: from_utf8_unchecked (# bytes) } } } } }
    };
}

impl_79!();