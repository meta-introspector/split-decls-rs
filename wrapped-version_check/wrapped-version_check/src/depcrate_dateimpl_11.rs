// Generated macro for impl_11 (impl)
macro_rules! Depcrate_dateimpl_11 {
() => {
// Module: crate::date
// Provides: {"impl_11"}
// Dependencies: {}
impl fmt :: Display for Date { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let (y , m , d) = self . to_ymd () ; write ! (f , "{}-{:02}-{:02}" , y , m , d) } }
};
}
