// Generated macro for Builder (struct)
macro_rules! Depcrate_schannel_credBuilder {
() => {
// Module: crate::schannel_cred
// Provides: {"Builder"}
// Dependencies: {}
# [doc = " A builder type for `SchannelCred`s."] # [derive (Default , Debug)] pub struct Builder { supported_algorithms : Option < Vec < Algorithm > > , enabled_protocols : Option < Vec < Protocol > > , certs : Vec < CertContext > , }
};
}
