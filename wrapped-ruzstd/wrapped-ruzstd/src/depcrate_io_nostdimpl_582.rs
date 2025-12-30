// Generated macro for impl_582 (impl)
macro_rules! Depcrate_io_nostdimpl_582 {
() => {
// Module: crate::io_nostd
// Provides: {"impl_582"}
// Dependencies: {}
impl alloc :: fmt :: Debug for Error { fn fmt (& self , f : & mut alloc :: fmt :: Formatter < '_ >) -> Result < () , alloc :: fmt :: Error > { let mut s = f . debug_struct ("Error") ; s . field ("kind" , & self . kind) ; if let Some (err) = self . err . as_ref () { s . field ("err" , & alloc :: format ! ("{err}")) ; } s . finish () } }
};
}
