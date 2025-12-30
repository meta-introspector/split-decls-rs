// Generated macro for impl_20 (impl)
macro_rules! Depcrate_ioimpl_20 {
() => {
// Module: crate::io
// Provides: {"impl_20"}
// Dependencies: {}
impl Handle { # [doc = " Sequence a `read` operation."] # [doc = ""] # [doc = " The next operation in the mock's script will be to expect a `read` call"] # [doc = " and return `buf`."] pub fn read (& mut self , buf : & [u8]) -> & mut Self { self . tx . send (Action :: Read (buf . into ())) . unwrap () ; self } # [doc = " Sequence a `read` operation error."] # [doc = ""] # [doc = " The next operation in the mock's script will be to expect a `read` call"] # [doc = " and return `error`."] pub fn read_error (& mut self , error : io :: Error) -> & mut Self { let error = Some (error . into ()) ; self . tx . send (Action :: ReadError (error)) . unwrap () ; self } # [doc = " Sequence a `write` operation."] # [doc = ""] # [doc = " The next operation in the mock's script will be to expect a `write`"] # [doc = " call."] pub fn write (& mut self , buf : & [u8]) -> & mut Self { self . tx . send (Action :: Write (buf . into ())) . unwrap () ; self } # [doc = " Sequence a `write` operation error."] # [doc = ""] # [doc = " The next operation in the mock's script will be to expect a `write`"] # [doc = " call error."] pub fn write_error (& mut self , error : io :: Error) -> & mut Self { let error = Some (error . into ()) ; self . tx . send (Action :: WriteError (error)) . unwrap () ; self } }
};
}
