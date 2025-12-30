// Generated macro for impl_1071 (impl)
macro_rules! Depcrate_io_errorimpl_1071 {
() => {
// Module: crate::io::error
// Provides: {"impl_1071"}
// Dependencies: {}
impl fmt :: Debug for Repr { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . data () { ErrorData :: Os (code) => fmt . debug_struct ("Os") . field ("code" , & code) . field ("kind" , & sys :: decode_error_kind (code)) . field ("message" , & sys :: os :: error_string (code)) . finish () , ErrorData :: Custom (c) => fmt :: Debug :: fmt (& c , fmt) , ErrorData :: Simple (kind) => fmt . debug_tuple ("Kind") . field (& kind) . finish () , ErrorData :: SimpleMessage (msg) => fmt . debug_struct ("Error") . field ("kind" , & msg . kind) . field ("message" , & msg . message) . finish () , } } }
};
}
