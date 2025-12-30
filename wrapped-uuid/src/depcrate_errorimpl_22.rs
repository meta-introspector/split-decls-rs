// Generated macro for impl_22 (impl)
macro_rules! Depcrate_errorimpl_22 {
() => {
// Module: crate::error
// Provides: {"impl_22"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self . 0 { ErrorKind :: ParseChar { character , index , .. } => { write ! (f , "invalid character: expected an optional prefix of `urn:uuid:` followed by [0-9a-fA-F-], found `{}` at {}" , character , index) } ErrorKind :: ParseSimpleLength { len } => { write ! (f , "invalid length: expected length 32 for simple format, found {}" , len) } ErrorKind :: ParseByteLength { len } => { write ! (f , "invalid length: expected 16 bytes, found {}" , len) } ErrorKind :: ParseGroupCount { count } => { write ! (f , "invalid group count: expected 5, found {}" , count) } ErrorKind :: ParseGroupLength { group , len , .. } => { let expected = [8 , 4 , 4 , 4 , 12] [group] ; write ! (f , "invalid group length in group {}: expected {}, found {}" , group , expected , len) } ErrorKind :: ParseInvalidUTF8 => write ! (f , "non-UTF8 input") , ErrorKind :: Nil => write ! (f , "the UUID is nil") , ErrorKind :: ParseOther => write ! (f , "failed to parse a UUID") , # [cfg (feature = "std")] ErrorKind :: InvalidSystemTime (ref e) => { write ! (f , "the system timestamp is invalid: {e}") } } } }
};
}
