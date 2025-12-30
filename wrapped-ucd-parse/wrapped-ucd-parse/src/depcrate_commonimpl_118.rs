// Generated macro for impl_118 (impl)
macro_rules! Depcrate_commonimpl_118 {
() => {
// Module: crate::common
// Provides: {"impl_118"}
// Dependencies: {}
impl < R : io :: Read , D > UcdLineParser < R , D > { # [doc = " Create a new parser that parses the reader given."] # [doc = ""] # [doc = " The type of data parsed is determined when the `parse_next` function"] # [doc = " is called by virtue of the type requested."] # [doc = ""] # [doc = " Note that the reader is buffered internally, so the caller does not"] # [doc = " need to provide their own buffering."] pub (crate) fn new (path : Option < PathBuf > , rdr : R) -> UcdLineParser < R , D > { UcdLineParser { path , rdr : io :: BufReader :: new (rdr) , line : String :: new () , line_number : 0 , _data : std :: marker :: PhantomData , } } }
};
}
