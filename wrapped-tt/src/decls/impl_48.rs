macro_rules! deps {
    () => {
        Literal!();
        LitKind!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < S > fmt :: Display for Literal < S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . kind { LitKind :: Byte => write ! (f , "b'{}'" , self . symbol) , LitKind :: Char => write ! (f , "'{}'" , self . symbol) , LitKind :: Integer | LitKind :: Float | LitKind :: Err (_) => write ! (f , "{}" , self . symbol) , LitKind :: Str => write ! (f , "\"{}\"" , self . symbol) , LitKind :: ByteStr => write ! (f , "b\"{}\"" , self . symbol) , LitKind :: CStr => write ! (f , "c\"{}\"" , self . symbol) , LitKind :: StrRaw (num_of_hashes) => { let num_of_hashes = num_of_hashes as usize ; write ! (f , r#"r{0:#<num_of_hashes$}"{text}"{0:#<num_of_hashes$}"# , "" , text = self . symbol) } LitKind :: ByteStrRaw (num_of_hashes) => { let num_of_hashes = num_of_hashes as usize ; write ! (f , r#"br{0:#<num_of_hashes$}"{text}"{0:#<num_of_hashes$}"# , "" , text = self . symbol) } LitKind :: CStrRaw (num_of_hashes) => { let num_of_hashes = num_of_hashes as usize ; write ! (f , r#"cr{0:#<num_of_hashes$}"{text}"{0:#<num_of_hashes$}"# , "" , text = self . symbol) } } ? ; if let Some (suffix) = & self . suffix { write ! (f , "{suffix}") ? ; } Ok (()) } }
    };
}

impl_48!()