// Generated macro for impl_48 (impl)
macro_rules! Depcrate_bufreadimpl_48 {
() => {
// Module: crate::bufread
// Provides: {"impl_48"}
// Dependencies: {}
impl < R : BufRead > Read for XzEncoder < R > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { loop { let (read , consumed , eof , ret) ; { let input = self . obj . fill_buf () ? ; eof = input . is_empty () ; let before_out = self . data . total_out () ; let before_in = self . data . total_in () ; let action = if eof { Action :: Finish } else { Action :: Run } ; ret = self . data . process (input , buf , action) ; read = (self . data . total_out () - before_out) as usize ; consumed = (self . data . total_in () - before_in) as usize ; } self . obj . consume (consumed) ; ret . unwrap () ; if read == 0 && ! eof && buf . len () > 0 { continue ; } return Ok (read) ; } } }
};
}
