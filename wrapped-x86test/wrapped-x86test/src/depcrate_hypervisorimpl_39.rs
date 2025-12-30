// Generated macro for impl_39 (impl)
macro_rules! Depcrate_hypervisorimpl_39 {
() => {
// Module: crate::hypervisor
// Provides: {"impl_39"}
// Dependencies: {}
impl Write for SerialPrinter { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { assert ! (buf . len () == 1) ; self . buffer . push (buf [0] as char) ; match buf [0] as char { '\n' => { std :: io :: stdout () . write (self . buffer . as_bytes ()) ? ; self . buffer . clear () ; } _ => { } } Ok (1) } fn flush (& mut self) -> io :: Result < () > { std :: io :: stdout () . write (self . buffer . as_bytes ()) ? ; self . buffer . clear () ; Ok (()) } }
};
}
