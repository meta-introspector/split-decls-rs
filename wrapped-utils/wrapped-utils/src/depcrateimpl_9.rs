// Generated macro for impl_9 (impl)
macro_rules! Depcrateimpl_9 {
() => {
// Module: crate
// Provides: {"impl_9"}
// Dependencies: {}
impl < W , const N : usize > LineBuffered < W , N > where W : uWrite , { # [doc = " Creates a new `LineBuffered` adapter"] pub fn new (writer : W) -> Self { Self { buffer : String :: new () , writer , } } # [doc = " Flushes the contents of the buffer"] pub fn flush (& mut self) -> Result < () , W :: Error > { let ret = self . writer . write_str (& self . buffer) ; self . buffer . clear () ; ret } # [doc = " Destroys the adapter and returns the underlying writer"] pub fn free (self) -> W { self . writer } fn push_str (& mut self , s : & str) -> Result < () , W :: Error > { let len = s . as_bytes () . len () ; if self . buffer . len () + len > self . buffer . capacity () { self . flush () ? ; } if len > self . buffer . capacity () { self . writer . write_str (s) ? ; } else { self . buffer . push_str (s) . unwrap_or_else (| _ | unsafe { assume_unreachable ! () }) } Ok (()) } }
};
}
