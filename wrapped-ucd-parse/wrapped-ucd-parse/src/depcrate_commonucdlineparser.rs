// Generated macro for UcdLineParser (struct)
macro_rules! Depcrate_commonUcdLineParser {
() => {
// Module: crate::common
// Provides: {"UcdLineParser"}
// Dependencies: {}
# [doc = " A line oriented parser for a particular UCD file."] # [doc = ""] # [doc = " Callers can build a line parser via the"] # [doc = " [`UcdFile::from_dir`](trait.UcdFile.html) method."] # [doc = ""] # [doc = " The `R` type parameter refers to the underlying `io::Read` implementation"] # [doc = " from which the UCD data is read."] # [doc = ""] # [doc = " The `D` type parameter refers to the type of the record parsed out of each"] # [doc = " line."] # [derive (Debug)] pub struct UcdLineParser < R , D > { path : Option < PathBuf > , rdr : io :: BufReader < R > , line : String , line_number : u64 , _data : std :: marker :: PhantomData < D > , }
};
}
