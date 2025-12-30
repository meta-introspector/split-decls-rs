// Generated macro for impl_39 (impl)
macro_rules! Depcrateimpl_39 {
() => {
// Module: crate
// Provides: {"impl_39"}
// Dependencies: {}
impl < S > FlameLayer < S , BufWriter < File > > where S : Subscriber + for < 'span > LookupSpan < 'span > , { # [doc = " Constructs a `FlameLayer` that outputs to a `BufWriter` to the given path, and a"] # [doc = " `FlushGuard` to ensure the writer is flushed."] pub fn with_file (path : impl AsRef < Path >) -> Result < (Self , FlushGuard < BufWriter < File > >) , Error > { let path = path . as_ref () ; let file = File :: create (path) . map_err (| source | Kind :: CreateFile { path : path . into () , source , }) . map_err (Error) ? ; let writer = BufWriter :: new (file) ; let layer = Self :: new (writer) ; let guard = layer . flush_on_drop () ; Ok ((layer , guard)) } }
};
}
