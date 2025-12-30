// Generated macro for PaxExtensions (struct)
macro_rules! Depcrate_paxPaxExtensions {
() => {
// Module: crate::pax
// Provides: {"PaxExtensions"}
// Dependencies: {}
# [doc = " An iterator over the pax extensions in an archive entry."] # [doc = ""] # [doc = " This iterator yields structures which can themselves be parsed into"] # [doc = " key/value pairs."] pub struct PaxExtensions < 'entry > { data : slice :: Split < 'entry , u8 , fn (& u8) -> bool > , }
};
}
