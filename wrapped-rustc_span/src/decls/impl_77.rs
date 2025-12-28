macro_rules! deps {
    () => {
        SourceFileHash!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl Display for SourceFileHash { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}=" , self . kind) ? ; for byte in self . value [0 .. self . hash_len ()] . into_iter () { write ! (f , "{byte:02x}") ? ; } Ok (()) } }
    };
}

impl_77!()