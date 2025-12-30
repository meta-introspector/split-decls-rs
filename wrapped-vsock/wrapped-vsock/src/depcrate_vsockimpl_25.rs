// Generated macro for impl_25 (impl)
macro_rules! Depcrate_vsockimpl_25 {
() => {
// Module: crate::vsock
// Provides: {"impl_25"}
// Dependencies: {}
impl Write for VsockStream { # [cfg (target_os = "hermit")] fn write (& mut self , buf : & [u8]) -> Result < usize > { let result = unsafe { check (write (self . fd . as_raw_fd () , buf . as_ptr () , buf . len ())) ? } ; Ok (result . try_into () . unwrap ()) } # [cfg (unix)] fn write (& mut self , buf : & [u8]) -> Result < usize > { let result : isize = unsafe { check (write (self . fd . as_raw_fd () , buf . as_ptr () as * const c_void , buf . len () ,)) ? } ; Ok (result . try_into () . unwrap ()) } fn flush (& mut self) -> Result < () > { Ok (()) } }
};
}
