// Generated macro for impl_26 (impl)
macro_rules! Depcrateimpl_26 {
() => {
// Module: crate
// Provides: {"impl_26"}
// Dependencies: {}
impl From < & mut File > for Bom { # [doc = " Detect the BOM type from a `File` instance."] # [doc = ""] # [doc = " Note that I/O errors are swallowed by this method."] # [doc = " Instead the default type, `Bom::Null`,"] # [doc = " will be returned."] fn from (file : & mut File) -> Self { let mut data = [0u8 ; 4] ; let mut result = file . read_exact (& mut data) ; if let Err (ref error) = result { if error . kind () == ErrorKind :: UnexpectedEof { let short_data = [0u8 ; 3] ; result = file . read_exact (& mut data) ; if let Err (ref error) = result { if error . kind () == ErrorKind :: UnexpectedEof { let short_data = [0u8 ; 2] ; result = file . read_exact (& mut data) ; data [0] = short_data [0] ; data [1] = short_data [1] ; } } else { data [0] = short_data [0] ; data [1] = short_data [1] ; data [2] = short_data [2] ; } } } if result . is_ok () { Bom :: from (& data [0 ..]) } else { Bom :: Null } } }
};
}
