// Generated macro for impl_55 (impl)
macro_rules! Depcrate_ownedimpl_55 {
() => {
// Module: crate::owned
// Provides: {"impl_55"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Error :: InvalidCodepoint (cp) => write ! (f , "could not construct trie set containing an \
                 invalid Unicode codepoint: 0x{:X}" , cp) , Error :: GaveUp => { write ! (f , "could not compress codepoint set into a trie") } } } }
};
}
