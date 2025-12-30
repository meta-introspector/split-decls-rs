// Generated macro for cert_entry (function)
macro_rules! Depcrate_ctl_contextcert_entry {
() => {
// Module: crate::ctl_context
// Provides: {"cert_entry"}
// Dependencies: {}
fn cert_entry (cert : & CertContext) -> io :: Result < Vec < u8 > > { unsafe { let mut size : u32 = 0 ; let res = Cryptography :: CertCreateCTLEntryFromCertificateContextProperties (cert . as_inner () , 0 , ptr :: null () , Cryptography :: CTL_ENTRY_FROM_PROP_CHAIN_FLAG , ptr :: null_mut () , ptr :: null_mut () , & mut size ,) ; if res == 0 { return Err (io :: Error :: last_os_error ()) ; } let mut entry = vec ! [0u8 ; size as usize] ; let res = Cryptography :: CertCreateCTLEntryFromCertificateContextProperties (cert . as_inner () , 0 , ptr :: null () , Cryptography :: CTL_ENTRY_FROM_PROP_CHAIN_FLAG , ptr :: null_mut () , entry . as_mut_ptr () as * mut Cryptography :: CTL_ENTRY , & mut size ,) ; if res == 0 { Err (io :: Error :: last_os_error ()) } else { Ok (entry) } } }
};
}
