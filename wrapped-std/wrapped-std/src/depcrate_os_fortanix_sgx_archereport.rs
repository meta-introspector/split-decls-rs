// Generated macro for ereport (function)
macro_rules! Depcrate_os_fortanix_sgx_archereport {
() => {
// Module: crate::os::fortanix_sgx::arch
// Provides: {"ereport"}
// Dependencies: {}
# [doc = " Call the `EREPORT` instruction."] # [doc = ""] # [doc = " This creates a cryptographic report describing the contents of the current"] # [doc = " enclave. The report may be verified by the enclave described in"] # [doc = " `targetinfo`."] # [unstable (feature = "sgx_platform" , issue = "56975")] pub fn ereport (targetinfo : & Align512 < [u8 ; 512] > , reportdata : & Align128 < [u8 ; 64] > ,) -> Align512 < [u8 ; 432] > { unsafe { let mut report = MaybeUninit :: uninit () ; asm ! ("xchg %rbx, {0}" , "enclu" , "mov {0}, %rbx" , inout (reg) targetinfo => _ , in ("eax") ENCLU_EREPORT , in ("rcx") reportdata , in ("rdx") report . as_mut_ptr () , options (att_syntax , preserves_flags , nostack) ,) ; report . assume_init () } }
};
}
