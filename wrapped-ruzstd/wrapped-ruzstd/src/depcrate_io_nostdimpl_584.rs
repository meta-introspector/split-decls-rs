// Generated macro for impl_584 (impl)
macro_rules! Depcrate_io_nostdimpl_584 {
() => {
// Module: crate::io_nostd
// Provides: {"impl_584"}
// Dependencies: {}
impl core :: fmt :: Display for Error { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . write_str (self . kind . as_str ()) ? ; if let Some (ref e) = self . err { e . fmt (f) ? ; } Ok (()) } }
};
}
