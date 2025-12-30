// Generated macro for SipHasher (struct)
macro_rules! Depcrate_sip128SipHasher {
() => {
// Module: crate::sip128
// Provides: {"SipHasher"}
// Dependencies: {}
# [doc = " An implementation of SipHash128 2-4."] # [doc = ""] # [doc = " SipHash is a general-purpose hashing function: it runs at a good"] # [doc = " speed (competitive with Spooky and City) and permits strong _keyed_"] # [doc = " hashing. This lets you key your hashtables from a strong RNG, such as"] # [doc = " [`rand::os::OsRng`](https://doc.rust-lang.org/rand/rand/os/struct.OsRng.html)."] # [doc = ""] # [doc = " Although the SipHash algorithm is considered to be generally strong,"] # [doc = " it is not intended for cryptographic purposes. As such, all"] # [doc = " cryptographic uses of this implementation are _strongly discouraged_."] # [derive (Debug , Clone , Copy , Default)] pub struct SipHasher (SipHasher24) ;
};
}
