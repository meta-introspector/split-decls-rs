// Generated macro for impl_102 (impl)
macro_rules! Depcrateimpl_102 {
() => {
// Module: crate
// Provides: {"impl_102"}
// Dependencies: {}
impl fmt :: Display for ErrorInner { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { use self :: ErrorInner :: * ; match self { DuplicateExtraField (key) => write ! (f , "duplicate extra field key {:?}" , key) , DuplicateHttpHeader (name) => write ! (f , "duplicate HTTP header {:?}" , name) , DuplicateLabel (key) => write ! (f , "duplicate label key {:?}" , key) , InvalidHttpHeaderName (name) => write ! (f , "invalid HTTP header name {:?}" , name) , InvalidHttpHeaderValue (name) => write ! (f , "invalid HTTP header value for {:?}" , name) , InvalidLabelCharacter (key , c) => { write ! (f , "invalid label character {:?} in key {:?}" , c , key) } InvalidLokiUrl => write ! (f , "invalid Loki URL") , ReservedLabelLevel => write ! (f , "cannot add custom label for \"level\"") , } } }
};
}
