// Generated macro for impl_95 (impl)
macro_rules! Depcrate_variablesimpl_95 {
() => {
// Module: crate::variables
// Provides: {"impl_95"}
// Dependencies: {}
impl Display for Variables { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { f . write_str ("{") ? ; for (i , (name , value)) in self . 0 . iter () . enumerate () { write ! (f , "{}{}: {}" , if i == 0 { "" } else { ", " } , name , value) ? ; } f . write_str ("}") } }
};
}
