// Generated macro for read_pubkey (function)
macro_rules! Depcrateread_pubkey {
() => {
// Module: crate
// Provides: {"read_pubkey"}
// Dependencies: {}
pub fn read_pubkey (current : & mut usize , data : & [u8]) -> Result < Pubkey , SanitizeError > { let len = std :: mem :: size_of :: < Pubkey > () ; if data . len () < * current + len { return Err (SanitizeError :: IndexOutOfBounds) ; } let e = Pubkey :: try_from (& data [* current .. * current + len]) . map_err (| _ | SanitizeError :: ValueOutOfBounds) ? ; * current += len ; Ok (e) }
};
}
