// Generated macro for impl_159 (impl)
macro_rules! Depcrateimpl_159 {
() => {
// Module: crate
// Provides: {"impl_159"}
// Dependencies: {}
# [cfg (feature = "std")] impl From < std :: io :: Error > for Error { fn from (e : std :: io :: Error) -> Self { match e . kind () { std :: io :: ErrorKind :: UnexpectedEof => Self :: EndOfStream , _ => Self :: DecodingError (format ! ("io error: {e:?}")) , } } }
};
}
