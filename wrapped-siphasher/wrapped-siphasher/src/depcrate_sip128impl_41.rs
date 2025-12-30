// Generated macro for impl_41 (impl)
macro_rules! Depcrate_sip128impl_41 {
() => {
// Module: crate::sip128
// Provides: {"impl_41"}
// Dependencies: {}
impl PartialEq for Hash128 { # [doc = " Constant-time equality comparison to prevent timing attacks."] fn eq (& self , other : & Self) -> bool { let x = (self . h1 ^ other . h1) | (self . h2 ^ other . h2) ; unsafe { core :: ptr :: read_volatile (& x) == 0 } } }
};
}
