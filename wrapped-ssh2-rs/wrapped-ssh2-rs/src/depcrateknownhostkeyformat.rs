// Generated macro for KnownHostKeyFormat (enum)
macro_rules! DepcrateKnownHostKeyFormat {
() => {
// Module: crate
// Provides: {"KnownHostKeyFormat"}
// Dependencies: {}
# [allow (missing_docs)] # [derive (Copy , Clone , Debug)] pub enum KnownHostKeyFormat { Unknown = raw :: LIBSSH2_KNOWNHOST_KEY_UNKNOWN as isize , Rsa1 = raw :: LIBSSH2_KNOWNHOST_KEY_RSA1 as isize , SshRsa = raw :: LIBSSH2_KNOWNHOST_KEY_SSHRSA as isize , SshDss = raw :: LIBSSH2_KNOWNHOST_KEY_SSHDSS as isize , Ecdsa256 = raw :: LIBSSH2_KNOWNHOST_KEY_ECDSA_256 as isize , Ecdsa384 = raw :: LIBSSH2_KNOWNHOST_KEY_ECDSA_384 as isize , Ecdsa521 = raw :: LIBSSH2_KNOWNHOST_KEY_ECDSA_521 as isize , Ed25519 = raw :: LIBSSH2_KNOWNHOST_KEY_ED25519 as isize , }
};
}
