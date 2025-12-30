// Generated macro for impl_514 (impl)
macro_rules! Depcrate_io_nostdimpl_514 {
() => {
// Module: crate::io_nostd
// Provides: {"impl_514"}
// Dependencies: {}
impl core :: fmt :: Display for Error { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . write_str (self . kind . as_str ()) ? ; if let Some (ref e) = self . err { e . fmt (f) ? ; } Ok (()) } }
};
}
