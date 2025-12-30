// Generated macro for test (module)
macro_rules! Depcrate_os_macos_encrypt_transformtest {
() => {
// Module: crate::os::macos::encrypt_transform
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use hex :: FromHex ; use super :: * ; use crate :: os :: macos :: item :: KeyType ; use crate :: os :: macos :: key :: SecKeyExt ; # [test] fn cbc_mmt_256 () { let key = "87725bd43a45608814180773f0e7ab95a3c859d83a2130e884190e44d14c6996" ; let iv = "e49651988ebbb72eb8bb80bb9abbca34" ; let ciphertext = "5b97a9d423f4b97413f388d9a341e727bb339f8e18a3fac2f2fb85abdc8f135deb30054a\
                          1afdc9b6ed7da16c55eba6b0d4d10c74e1d9a7cf8edfaeaa684ac0bd9f9d24ba674955c7\
                          9dc6be32aee1c260b558ff07e3a4d49d24162011ff254db8be078e8ad07e648e6bf56793\
                          76cb4321a5ef01afe6ad8816fcc7634669c8c4389295c9241e45fff39f3225f7745032da\
                          eebe99d4b19bcb215d1bfdb36eda2c24" ; let plaintext = "bfe5c6354b7a3ff3e192e05775b9b75807de12e38a626b8bf0e12d5fff78e4f1775aa7d79\
                         2d885162e66d88930f9c3b2cdf8654f56972504803190386270f0aa43645db187af41fcea\
                         639b1f8026ccdd0c23e0de37094a8b941ecb7602998a4b2604e69fc04219585d854600e0a\
                         d6f99a53b2504043c08b1c3e214d17cde053cbdf91daa999ed5b47c37983ba3ee254bc5c7\
                         93837daaa8c85cfc12f7f54f699f" ; let key = Vec :: < u8 > :: from_hex (key) . unwrap () ; let key = CFData :: from_buffer (& key) ; let key = SecKey :: from_data (KeyType :: aes () , & key) . unwrap () ; let iv = Vec :: < u8 > :: from_hex (iv) . unwrap () ; let ciphertext = Vec :: < u8 > :: from_hex (ciphertext) . unwrap () ; let plaintext = Vec :: < u8 > :: from_hex (plaintext) . unwrap () ; let decrypted = Builder :: new () . padding (Padding :: none ()) . iv (CFData :: from_buffer (& iv)) . decrypt (& key , & CFData :: from_buffer (& ciphertext)) . unwrap () ; assert_eq ! (plaintext , decrypted . bytes ()) ; let encrypted = Builder :: new () . padding (Padding :: none ()) . iv (CFData :: from_buffer (& iv)) . encrypt (& key , & CFData :: from_buffer (& plaintext)) . unwrap () ; assert_eq ! (ciphertext , encrypted . bytes ()) ; } }
};
}
