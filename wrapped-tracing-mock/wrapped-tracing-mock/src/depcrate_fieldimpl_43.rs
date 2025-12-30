// Generated macro for impl_43 (impl)
macro_rules! Depcrate_fieldimpl_43 {
() => {
// Module: crate::field
// Provides: {"impl_43"}
// Dependencies: {}
impl fmt :: Display for ExpectedValue { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { ExpectedValue :: F64 (v) => write ! (f , "f64 = {:?}" , v) , ExpectedValue :: I64 (v) => write ! (f , "i64 = {:?}" , v) , ExpectedValue :: U64 (v) => write ! (f , "u64 = {:?}" , v) , ExpectedValue :: Bool (v) => write ! (f , "bool = {:?}" , v) , ExpectedValue :: Str (v) => write ! (f , "&str = {:?}" , v) , ExpectedValue :: Debug (v) => write ! (f , "&fmt::Debug = {:?}" , v) , ExpectedValue :: Any => write ! (f , "_ = _") , } } }
};
}
