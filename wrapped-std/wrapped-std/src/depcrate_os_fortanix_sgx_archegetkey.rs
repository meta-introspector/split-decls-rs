// Generated macro for egetkey (function)
macro_rules! Depcrate_os_fortanix_sgx_archegetkey {
() => {
// Module: crate::os::fortanix_sgx::arch
// Provides: {"egetkey"}
// Dependencies: {}
# [doc = " Call the `EGETKEY` instruction to obtain a 128-bit secret key."] # [unstable (feature = "sgx_platform" , issue = "56975")] pub fn egetkey (request : & Align512 < [u8 ; 512] >) -> Result < Align16 < [u8 ; 16] > , u32 > { unsafe { let mut out = MaybeUninit :: uninit () ; let error ; asm ! ("xchg %rbx, {0}" , "enclu" , "mov {0}, %rbx" , inout (reg) request => _ , inlateout ("eax") ENCLU_EGETKEY => error , in ("rcx") out . as_mut_ptr () , options (att_syntax , nostack) ,) ; match error { 0 => Ok (out . assume_init ()) , err => Err (err) , } } }
};
}
