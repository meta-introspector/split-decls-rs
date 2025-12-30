// Generated macro for impl_89 (impl)
macro_rules! Depcrate_writeimpl_89 {
() => {
// Module: crate::write
// Provides: {"impl_89"}
// Dependencies: {}
impl < W : Write > Write for XzEncoder < W > { fn write (& mut self , data : & [u8]) -> io :: Result < usize > { loop { self . dump () ? ; let total_in = self . total_in () ; self . data . process_vec (data , & mut self . buf , Action :: Run) . unwrap () ; let written = (self . total_in () - total_in) as usize ; if written > 0 || data . len () == 0 { return Ok (written) ; } } } fn flush (& mut self) -> io :: Result < () > { loop { self . dump () ? ; let status = self . data . process_vec (& [] , & mut self . buf , Action :: FullFlush) . unwrap () ; if status == Status :: StreamEnd { break ; } } self . obj . as_mut () . unwrap () . flush () } }
};
}
