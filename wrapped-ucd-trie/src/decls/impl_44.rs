macro_rules! deps {
    () => {
        Error!();
        Result!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Error :: InvalidCodepoint (cp) => write ! (f , "could not construct trie set containing an \
                 invalid Unicode codepoint: 0x{:X}" , cp) , Error :: GaveUp => { write ! (f , "could not compress codepoint set into a trie") } } } }
    };
}

impl_44!();