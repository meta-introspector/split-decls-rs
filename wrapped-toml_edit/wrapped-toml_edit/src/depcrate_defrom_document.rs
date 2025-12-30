// Generated macro for from_document (function)
macro_rules! Depcrate_defrom_document {
() => {
// Module: crate::de
// Provides: {"from_document"}
// Dependencies: {}
# [doc = " Convert a [`DocumentMut`][crate::DocumentMut] into `T`."] pub fn from_document < T > (d : impl Into < Deserializer >) -> Result < T , Error > where T : DeserializeOwned , { let deserializer = d . into () ; T :: deserialize (deserializer) }
};
}
