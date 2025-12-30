// Generated macro for PartiallyExtractedSecrets (struct)
macro_rules! Depcrate_suitesPartiallyExtractedSecrets {
() => {
// Module: crate::suites
// Provides: {"PartiallyExtractedSecrets"}
// Dependencies: {}
# [doc = " [ExtractedSecrets] minus the sequence numbers"] pub (crate) struct PartiallyExtractedSecrets { # [doc = " secrets for the \"tx\" (transmit) direction"] pub (crate) tx : ConnectionTrafficSecrets , # [doc = " secrets for the \"rx\" (receive) direction"] pub (crate) rx : ConnectionTrafficSecrets , }
};
}
