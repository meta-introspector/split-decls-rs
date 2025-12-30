// Generated macro for impl_49 (impl)
macro_rules! Depcrate_cert_contextimpl_49 {
() => {
// Module: crate::cert_context
// Provides: {"impl_49"}
// Dependencies: {}
impl < 'a > AcquirePrivateKeyOptions < 'a > { # [doc = " If set, the certificate's public key will be compared with the private key to ensure a"] # [doc = " match."] pub fn compare_key (& mut self , compare_key : bool) -> & mut AcquirePrivateKeyOptions < 'a > { self . flag (Cryptography :: CRYPT_ACQUIRE_COMPARE_KEY_FLAG , compare_key) } # [doc = " If set, the lookup will not display any user interface, even if that causes the lookup to"] # [doc = " fail."] pub fn silent (& mut self , silent : bool) -> & mut AcquirePrivateKeyOptions < 'a > { self . flag (Cryptography :: CRYPT_ACQUIRE_SILENT_FLAG , silent) } fn flag (& mut self , flag : u32 , set : bool) -> & mut AcquirePrivateKeyOptions < 'a > { if set { self . flags |= flag ; } else { self . flags &= ! flag ; } self } # [doc = " Acquires the private key handle."] pub fn acquire (& self) -> io :: Result < PrivateKey > { unsafe { let flags = self . flags | Cryptography :: CRYPT_ACQUIRE_ALLOW_NCRYPT_KEY_FLAG ; let mut handle = Cryptography :: HCRYPTPROV_OR_NCRYPT_KEY_HANDLE :: default () ; let mut spec = Cryptography :: CERT_KEY_SPEC :: default () ; let mut free = windows_sys :: core :: BOOL :: default () ; let res = Cryptography :: CryptAcquireCertificatePrivateKey (self . cert . 0 , flags , ptr :: null_mut () , & mut handle , & mut spec , & mut free ,) ; if res == 0 { return Err (io :: Error :: last_os_error ()) ; } assert_ne ! (free , 0) ; if spec & Cryptography :: CERT_NCRYPT_KEY_SPEC != 0 { Ok (PrivateKey :: NcryptKey (NcryptKey :: from_inner (handle))) } else { Ok (PrivateKey :: CryptProv (CryptProv :: from_inner (handle))) } } } }
};
}
