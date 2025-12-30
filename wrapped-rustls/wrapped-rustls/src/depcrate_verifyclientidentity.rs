// Generated macro for ClientIdentity (struct)
macro_rules! Depcrate_verifyClientIdentity {
() => {
// Module: crate::verify
// Provides: {"ClientIdentity"}
// Dependencies: {}
# [doc = " Data required to verify a client's identity."] # [non_exhaustive] # [derive (Debug)] pub struct ClientIdentity < 'a > { # [doc = " Identity information presented by the client."] pub identity : & 'a Identity < 'a > , # [doc = " Current time against which time-sensitive inputs should be validated."] pub now : UnixTime , }
};
}
