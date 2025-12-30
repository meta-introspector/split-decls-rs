// Generated macro for impl_63 (impl)
macro_rules! Depcrateimpl_63 {
() => {
// Module: crate
// Provides: {"impl_63"}
// Dependencies: {}
# [cfg (windows)] impl io :: Write for WindowsBuffer { # [inline] fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . buf . extend_from_slice (buf) ; Ok (buf . len ()) } # [inline] fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
};
}
