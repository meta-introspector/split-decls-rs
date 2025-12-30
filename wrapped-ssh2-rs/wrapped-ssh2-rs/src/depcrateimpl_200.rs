// Generated macro for impl_200 (impl)
macro_rules! Depcrateimpl_200 {
() => {
// Module: crate
// Provides: {"impl_200"}
// Dependencies: {}
impl From < HostKeyType > for KnownHostKeyFormat { fn from (host_type : HostKeyType) -> KnownHostKeyFormat { match host_type { HostKeyType :: Unknown => KnownHostKeyFormat :: Unknown , HostKeyType :: Rsa => KnownHostKeyFormat :: SshRsa , HostKeyType :: Dss => KnownHostKeyFormat :: SshDss , HostKeyType :: Ecdsa256 => KnownHostKeyFormat :: Ecdsa256 , HostKeyType :: Ecdsa384 => KnownHostKeyFormat :: Ecdsa384 , HostKeyType :: Ecdsa521 => KnownHostKeyFormat :: Ecdsa521 , HostKeyType :: Ed25519 => KnownHostKeyFormat :: Ed25519 , } } }
};
}
