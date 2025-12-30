// Generated macro for ReadExt (trait)
macro_rules! Depcrate_tendrilReadExt {
() => {
// Module: crate::tendril
// Provides: {"ReadExt"}
// Dependencies: {}
# [doc = " Extension trait for `io::Read`."] pub trait ReadExt : io :: Read { fn read_to_tendril < A > (& mut self , buf : & mut Tendril < fmt :: Bytes , A >) -> io :: Result < usize > where A : Atomicity ; }
};
}
