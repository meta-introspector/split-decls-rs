// Generated macro for seal_fully (function)
macro_rules! Depcrate_memfdseal_fully {
() => {
// Module: crate::memfd
// Provides: {"seal_fully"}
// Dependencies: {}
pub fn seal_fully (fd : RawFd) -> Result < () > { let all_seals = F_SEAL_SHRINK | F_SEAL_GROW | F_SEAL_WRITE | F_SEAL_SEAL ; let result = unsafe { fcntl (fd , F_ADD_SEALS , all_seals) } ; if result < 0 { Err (Error :: last_os_error ()) } else { Ok (()) } }
};
}
