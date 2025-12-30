// Generated macro for impl_24 (impl)
macro_rules! Depcrate_vsockimpl_24 {
() => {
// Module: crate::vsock
// Provides: {"impl_24"}
// Dependencies: {}
impl Read for VsockStream { # [cfg (target_os = "hermit")] fn read (& mut self , buf : & mut [u8]) -> Result < usize > { let result = unsafe { check (read (self . fd . as_raw_fd () , buf . as_mut_ptr () , buf . len ())) ? } ; Ok (result . try_into () . unwrap ()) } # [cfg (unix)] fn read (& mut self , buf : & mut [u8]) -> Result < usize > { let result = unsafe { check (read (self . fd . as_raw_fd () , buf . as_mut_ptr () as * mut c_void , buf . len () ,)) ? } ; Ok (result . try_into () . unwrap ()) } }
};
}
