// Generated macro for impl_336 (impl)
macro_rules! Depcrate_unicode_dataimpl_336 {
() => {
// Module: crate::unicode_data
// Provides: {"impl_336"}
// Dependencies: {}
impl std :: fmt :: Display for UnicodeDataNumeric { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match * self { UnicodeDataNumeric :: Integer (n) => write ! (f , "{}" , n) , UnicodeDataNumeric :: Rational (n , d) => write ! (f , "{}/{}" , n , d) , } } }
};
}
