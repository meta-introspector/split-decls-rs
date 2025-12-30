// Generated macro for impl_211 (impl)
macro_rules! Depcrate_resultimpl_211 {
() => {
// Module: crate::result
// Provides: {"impl_211"}
// Dependencies: {}
impl From < ZipError > for io :: Error { fn from (err : ZipError) -> io :: Error { let kind = match & err { ZipError :: Io (err) => err . kind () , ZipError :: InvalidArchive (_) => io :: ErrorKind :: InvalidData , ZipError :: UnsupportedArchive (_) => io :: ErrorKind :: Unsupported , ZipError :: FileNotFound => io :: ErrorKind :: NotFound , ZipError :: InvalidPassword => io :: ErrorKind :: InvalidInput , } ; io :: Error :: new (kind , err) } }
};
}
