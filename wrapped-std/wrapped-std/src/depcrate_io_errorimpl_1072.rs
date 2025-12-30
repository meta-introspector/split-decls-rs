// Generated macro for impl_1072 (impl)
macro_rules! Depcrate_io_errorimpl_1072 {
() => {
// Module: crate::io::error
// Provides: {"impl_1072"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl fmt :: Display for Error { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . repr . data () { ErrorData :: Os (code) => { let detail = sys :: os :: error_string (code) ; write ! (fmt , "{detail} (os error {code})") } ErrorData :: Custom (ref c) => c . error . fmt (fmt) , ErrorData :: Simple (kind) => write ! (fmt , "{}" , kind . as_str ()) , ErrorData :: SimpleMessage (msg) => msg . message . fmt (fmt) , } } }
};
}
