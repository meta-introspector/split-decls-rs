macro_rules! deps {
    () => {
        TomlError!();
        Error!();
    };
}

macro_rules! impl_286 {
    () => {
        deps!();
        impl Error { pub (crate) fn custom < T > (msg : T , span : Option < std :: ops :: Range < usize > >) -> Self where T : std :: fmt :: Display , { Self { inner : crate :: TomlError :: custom (msg . to_string () , span) , } } # [doc = " Add key while unwinding"] pub fn add_key (& mut self , key : String) { self . inner . add_key (key) ; } # [doc = " What went wrong"] pub fn message (& self) -> & str { self . inner . message () } # [doc = " The start/end index into the original document where the error occurred"] pub fn span (& self) -> Option < std :: ops :: Range < usize > > { self . inner . span () } pub (crate) fn set_span (& mut self , span : Option < std :: ops :: Range < usize > >) { self . inner . set_span (span) ; } # [doc = " Provide the encoded TOML the error applies to"] pub fn set_input (& mut self , input : Option < & str >) { self . inner . set_input (input) ; } }
    };
}

impl_286!();