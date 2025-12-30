// Generated macro for StaticSecret (struct)
macro_rules! DepcrateStaticSecret {
() => {
// Module: crate
// Provides: {"StaticSecret"}
// Dependencies: {}
# [doc = " A Diffie-Hellman secret key that can be used to compute multiple [`SharedSecret`]s."] # [doc = ""] # [doc = " This type is identical to the [`EphemeralSecret`] type, except that the"] # [doc = " [`StaticSecret::diffie_hellman`] method does not consume the secret key, and the type provides"] # [doc = " serialization methods to save and load key material.  This means that the secret may be used"] # [doc = " multiple times (but does not *have to be*)."] # [doc = ""] # [doc = " # Warning"] # [doc = ""] # [doc = " If you're uncertain about whether you should use this, then you likely"] # [doc = " should not be using this.  Our strongly recommended advice is to use"] # [doc = " [`EphemeralSecret`] at all times, as that type enforces at compile-time that"] # [doc = " secret keys are never reused, which can have very serious security"] # [doc = " implications for many protocols."] # [cfg (feature = "static_secrets")] # [derive (Clone , Zeroize)] # [zeroize (drop)] pub struct StaticSecret (Array < u8 , U56 >) ;
};
}
