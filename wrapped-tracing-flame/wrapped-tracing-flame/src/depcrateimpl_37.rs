// Generated macro for impl_37 (impl)
macro_rules! Depcrateimpl_37 {
() => {
// Module: crate
// Provides: {"impl_37"}
// Dependencies: {}
impl < W > FlushGuard < W > where W : Write + 'static , { # [doc = " Flush the internal writer of the `FlameLayer`, ensuring that all"] # [doc = " intermediately buffered contents reach their destination."] pub fn flush (& self) -> Result < () , Error > { let mut guard = match self . out . lock () { Ok (guard) => guard , Err (e) => { if ! std :: thread :: panicking () { panic ! ("{}" , e) ; } else { return Ok (()) ; } } } ; guard . flush () . map_err (Kind :: FlushFile) . map_err (Error) } }
};
}
