// Generated macro for impl_1789 (impl)
macro_rules! Depcrate_tls13_key_scheduleimpl_1789 {
() => {
// Module: crate::tls13::key_schedule
// Provides: {"impl_1789"}
// Dependencies: {}
impl SecretKind { fn to_bytes (self) -> & 'static [u8] { use self :: SecretKind :: * ; match self { ResumptionPskBinderKey => b"res binder" , ClientEarlyTrafficSecret => b"c e traffic" , EarlyExporterMasterSecret => b"e exp master" , ClientHandshakeTrafficSecret => b"c hs traffic" , ServerHandshakeTrafficSecret => b"s hs traffic" , ClientApplicationTrafficSecret => b"c ap traffic" , ServerApplicationTrafficSecret => b"s ap traffic" , ExporterMasterSecret => b"exp master" , ResumptionMasterSecret => b"res master" , DerivedSecret => b"derived" , ServerEchConfirmationSecret => b"ech accept confirmation" , ServerEchHrrConfirmationSecret => b"hrr ech accept confirmation" , } } fn log_label (self) -> Option < & 'static str > { use self :: SecretKind :: * ; Some (match self { ClientEarlyTrafficSecret => "CLIENT_EARLY_TRAFFIC_SECRET" , EarlyExporterMasterSecret => "EARLY_EXPORTER_SECRET" , ClientHandshakeTrafficSecret => "CLIENT_HANDSHAKE_TRAFFIC_SECRET" , ServerHandshakeTrafficSecret => "SERVER_HANDSHAKE_TRAFFIC_SECRET" , ClientApplicationTrafficSecret => "CLIENT_TRAFFIC_SECRET_0" , ServerApplicationTrafficSecret => "SERVER_TRAFFIC_SECRET_0" , ExporterMasterSecret => "EXPORTER_SECRET" , _ => { return None ; } }) } }
};
}
