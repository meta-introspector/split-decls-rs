// Generated macro for SecretKind (enum)
macro_rules! Depcrate_tls13_key_scheduleSecretKind {
() => {
// Module: crate::tls13::key_schedule
// Provides: {"SecretKind"}
// Dependencies: {}
# [doc = " The kinds of secret we can extract from `KeySchedule`."] # [derive (Debug , Clone , Copy , PartialEq)] enum SecretKind { ResumptionPskBinderKey , ClientEarlyTrafficSecret , EarlyExporterMasterSecret , ClientHandshakeTrafficSecret , ServerHandshakeTrafficSecret , ClientApplicationTrafficSecret , ServerApplicationTrafficSecret , ExporterMasterSecret , ResumptionMasterSecret , DerivedSecret , ServerEchConfirmationSecret , ServerEchHrrConfirmationSecret , }
};
}
